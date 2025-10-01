use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use server_fn::error::NoCustomError;
#[cfg(feature = "ssr")]
use sqlx::SqlitePool;

// Statistics data structures
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OverallStats {
    pub attendance_rate: f64,
    pub total_students: i64,
    pub total_classes: i64,
    pub absent_today: i64,
    pub avg_class_size: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WeeklyTrend {
    pub week: String,
    pub attendance_rate: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModuleAbsence {
    pub module_title: String,
    pub absence_rate: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModuleOption {
    pub module_code: i64,
    pub module_title: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClassOption {
    pub class_id: i64,
    pub title: String,
}

// Server function to get overall statistics with optional filters
#[server(GetOverallStats, "/api")]
pub async fn get_overall_stats(
    module_code: Option<i64>,
    class_id: Option<i64>,
) -> Result<OverallStats, ServerFnError<NoCustomError>> {
    let pool = use_context::<SqlitePool>()
        .ok_or_else(|| ServerFnError::<NoCustomError>::ServerError("Database pool not found".to_string()))?;
    
    // Get overall attendance rate
    let attendance_rate: f64 = if let Some(cid) = class_id {
        // Filter by specific class
        sqlx::query_scalar(
            r#"
            SELECT 
                COALESCE(
                    CAST(SUM(CASE WHEN status = 'present' THEN 1 ELSE 0 END) AS REAL) * 100.0 / 
                    NULLIF(CAST(COUNT(*) AS REAL), 0),
                    0.0
                )
            FROM attendance
            WHERE classID = ?
            "#
        )
        .bind(cid)
        .fetch_one(&pool)
        .await
        .unwrap_or(0.0)
    } else if let Some(mc) = module_code {
        // Filter by module
        sqlx::query_scalar(
            r#"
            SELECT 
                COALESCE(
                    CAST(SUM(CASE WHEN a.status = 'present' THEN 1 ELSE 0 END) AS REAL) * 100.0 / 
                    NULLIF(CAST(COUNT(*) AS REAL), 0),
                    0.0
                )
            FROM attendance a
            JOIN classes c ON a.classID = c.classID
            WHERE c.moduleCode = ?
            "#
        )
        .bind(mc)
        .fetch_one(&pool)
        .await
        .unwrap_or(0.0)
    } else {
        // No filters - query attendance directly
        sqlx::query_scalar(
            r#"
            SELECT 
                COALESCE(
                    CAST(SUM(CASE WHEN status = 'present' THEN 1 ELSE 0 END) AS REAL) * 100.0 / 
                    NULLIF(CAST(COUNT(*) AS REAL), 0),
                    0.0
                )
            FROM attendance
            "#
        )
        .fetch_one(&pool)
        .await
        .unwrap_or(0.0)
    };
    
    // Get total students
    let total_students: i64 = sqlx::query_scalar(
        r#"SELECT COUNT(*) FROM users WHERE role = 'student'"#
    )
    .fetch_one(&pool)
    .await
    .unwrap_or(0);
    
    // Get total classes (filtered)
    let total_classes: i64 = if let Some(cid) = class_id {
        sqlx::query_scalar("SELECT COUNT(*) FROM classes WHERE classID = ?")
            .bind(cid)
            .fetch_one(&pool)
            .await
            .unwrap_or(0)
    } else if let Some(mc) = module_code {
        sqlx::query_scalar("SELECT COUNT(*) FROM classes WHERE moduleCode = ?")
            .bind(mc)
            .fetch_one(&pool)
            .await
            .unwrap_or(0)
    } else {
        sqlx::query_scalar("SELECT COUNT(*) FROM classes")
            .fetch_one(&pool)
            .await
            .unwrap_or(0)
    };
    
    // Get absent today
    let absent_today: i64 = if let Some(cid) = class_id {
        sqlx::query_scalar(
            r#"
            SELECT COUNT(*)
            FROM attendance a
            JOIN classes c ON a.classID = c.classID
            WHERE c.date = date('now')
            AND a.status IN ('absent', 'late')
            AND a.classID = ?
            "#
        )
        .bind(cid)
        .fetch_one(&pool)
        .await
        .unwrap_or(0)
    } else if let Some(mc) = module_code {
        sqlx::query_scalar(
            r#"
            SELECT COUNT(*)
            FROM attendance a
            JOIN classes c ON a.classID = c.classID
            WHERE c.date = date('now')
            AND a.status IN ('absent', 'late')
            AND c.moduleCode = ?
            "#
        )
        .bind(mc)
        .fetch_one(&pool)
        .await
        .unwrap_or(0)
    } else {
        sqlx::query_scalar(
            r#"
            SELECT COUNT(*)
            FROM attendance a
            JOIN classes c ON a.classID = c.classID
            WHERE c.date = date('now')
            AND a.status IN ('absent', 'late')
            "#
        )
        .fetch_one(&pool)
        .await
        .unwrap_or(0)
    };
    
    // Get average class size
    let avg_class_size: f64 = if let Some(cid) = class_id {
        sqlx::query_scalar(
            r#"
            SELECT COALESCE(COUNT(DISTINCT studentID), 0)
            FROM attendance
            WHERE classID = ?
            "#
        )
        .bind(cid)
        .fetch_one(&pool)
        .await
        .map(|count: i64| count as f64)
        .unwrap_or(0.0)
    } else if let Some(mc) = module_code {
        sqlx::query_scalar(
            r#"
            SELECT COALESCE(AVG(student_count), 0.0)
            FROM (
                SELECT COUNT(DISTINCT a.studentID) as student_count
                FROM classes c
                LEFT JOIN attendance a ON c.classID = a.classID
                WHERE c.moduleCode = ?
                GROUP BY c.classID
            )
            "#
        )
        .bind(mc)
        .fetch_one(&pool)
        .await
        .unwrap_or(0.0)
    } else {
        sqlx::query_scalar(
            r#"
            SELECT COALESCE(AVG(student_count), 0.0)
            FROM (
                SELECT COUNT(DISTINCT a.studentID) as student_count
                FROM classes c
                LEFT JOIN attendance a ON c.classID = a.classID
                GROUP BY c.classID
            )
            "#
        )
        .fetch_one(&pool)
        .await
        .unwrap_or(0.0)
    };
    
    Ok(OverallStats {
        attendance_rate,
        total_students,
        total_classes,
        absent_today,
        avg_class_size,
    })
}

// Server function to get weekly attendance trends
#[server(GetWeeklyTrends, "/api")]
pub async fn get_weekly_trends(
    module_code: Option<i64>,
) -> Result<Vec<WeeklyTrend>, ServerFnError<NoCustomError>> {
    let pool = use_context::<SqlitePool>()
        .ok_or_else(|| ServerFnError::<NoCustomError>::ServerError("Database pool not found".to_string()))?;
    
    let query = if let Some(mc) = module_code {
        sqlx::query_as(
            r#"
            SELECT 
                'Week ' || strftime('%W', c.date) as week,
                COALESCE(
                    CAST(SUM(CASE WHEN a.status = 'present' THEN 1 ELSE 0 END) AS REAL) * 100.0 / 
                    NULLIF(CAST(COUNT(*) AS REAL), 0),
                    0.0
                ) as rate
            FROM classes c
            LEFT JOIN attendance a ON c.classID = a.classID
            WHERE c.moduleCode = ?
            GROUP BY strftime('%W', c.date)
            HAVING COUNT(a.attendanceID) > 0
            ORDER BY c.date DESC
            LIMIT 8
            "#
        )
        .bind(mc)
        .fetch_all(&pool)
        .await
        .unwrap_or_default()
    } else {
        sqlx::query_as(
            r#"
            SELECT 
                'Week ' || strftime('%W', c.date) as week,
                COALESCE(
                    CAST(SUM(CASE WHEN a.status = 'present' THEN 1 ELSE 0 END) AS REAL) * 100.0 / 
                    NULLIF(CAST(COUNT(*) AS REAL), 0),
                    0.0
                ) as rate
            FROM classes c
            LEFT JOIN attendance a ON c.classID = a.classID
            GROUP BY strftime('%W', c.date)
            HAVING COUNT(a.attendanceID) > 0
            ORDER BY c.date DESC
            LIMIT 8
            "#
        )
        .fetch_all(&pool)
        .await
        .unwrap_or_default()
    };
    
    let mut trends: Vec<WeeklyTrend> = query
        .into_iter()
        .map(|(week, rate): (String, f64)| WeeklyTrend {
            week,
            attendance_rate: rate,
        })
        .collect();
    
    // Reverse to show oldest to newest
    trends.reverse();
    
    Ok(trends)
}

// Server function to get most missed modules - NOW WITH FILTERS
#[server(GetMostMissedModules, "/api")]
pub async fn get_most_missed_modules(
    module_code: Option<i64>,
) -> Result<Vec<ModuleAbsence>, ServerFnError<NoCustomError>> {
    let pool = use_context::<SqlitePool>()
        .ok_or_else(|| ServerFnError::<NoCustomError>::ServerError("Database pool not found".to_string()))?;
    
    let rows: Vec<(String, f64)> = if let Some(mc) = module_code {
        // Filter by specific module
        sqlx::query_as(
            r#"
            SELECT 
                m.moduleTitle as title,
                COALESCE(
                    CAST(SUM(CASE WHEN a.status IN ('absent', 'late') THEN 1 ELSE 0 END) AS REAL) * 100.0 / 
                    NULLIF(CAST(COUNT(*) AS REAL), 0),
                    0.0
                ) as absence_rate
            FROM modules m
            JOIN classes c ON m.moduleCode = c.moduleCode
            LEFT JOIN attendance a ON c.classID = a.classID
            WHERE m.moduleCode = ?
            GROUP BY m.moduleCode, m.moduleTitle
            HAVING COUNT(a.attendanceID) > 0
            ORDER BY absence_rate DESC
            LIMIT 5
            "#
        )
        .bind(mc)
        .fetch_all(&pool)
        .await
        .unwrap_or_default()
    } else {
        // All modules
        sqlx::query_as(
            r#"
            SELECT 
                m.moduleTitle as title,
                COALESCE(
                    CAST(SUM(CASE WHEN a.status IN ('absent', 'late') THEN 1 ELSE 0 END) AS REAL) * 100.0 / 
                    NULLIF(CAST(COUNT(*) AS REAL), 0),
                    0.0
                ) as absence_rate
            FROM modules m
            JOIN classes c ON m.moduleCode = c.moduleCode
            LEFT JOIN attendance a ON c.classID = a.classID
            GROUP BY m.moduleCode, m.moduleTitle
            HAVING COUNT(a.attendanceID) > 0
            ORDER BY absence_rate DESC
            LIMIT 5
            "#
        )
        .fetch_all(&pool)
        .await
        .unwrap_or_default()
    };
    
    Ok(rows
        .into_iter()
        .map(|(title, rate)| ModuleAbsence {
            module_title: title,
            absence_rate: rate,
        })
        .collect())
}

// Server function to get module options for dropdown
#[server(GetModuleOptions, "/api")]
pub async fn get_module_options() -> Result<Vec<ModuleOption>, ServerFnError<NoCustomError>> {
    let pool = use_context::<SqlitePool>()
        .ok_or_else(|| ServerFnError::<NoCustomError>::ServerError("Database pool not found".to_string()))?;
    
    let rows: Vec<(i64, String)> = sqlx::query_as(
        r#"
        SELECT moduleCode, moduleTitle
        FROM modules
        ORDER BY moduleTitle
        "#
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_default();
    
    Ok(rows
        .into_iter()
        .map(|(code, title)| ModuleOption {
            module_code: code,
            module_title: title,
        })
        .collect())
}

// Server function to get class options for dropdown
#[server(GetClassOptions, "/api")]
pub async fn get_class_options(
    module_code: Option<i64>,
) -> Result<Vec<ClassOption>, ServerFnError<NoCustomError>> {
    let pool = use_context::<SqlitePool>()
        .ok_or_else(|| ServerFnError::<NoCustomError>::ServerError("Database pool not found".to_string()))?;
    
    let query = if let Some(mc) = module_code {
        sqlx::query_as(
            r#"
            SELECT classID, title
            FROM classes
            WHERE status IN ('upcoming', 'in_progress', 'completed')
            AND moduleCode = ?
            ORDER BY date DESC, time DESC
            "#
        )
        .bind(mc)
        .fetch_all(&pool)
        .await
        .unwrap_or_default()
    } else {
        vec![] // Return empty when no module selected
    };
    
    Ok(query
        .into_iter()
        .map(|(id, title): (i64, String)| ClassOption {
            class_id: id,
            title,
        })
        .collect())
}