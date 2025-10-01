#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use clock_it::app::*;
    use clock_it::database::{init_db_pool, test_db_connection, run_migrations, test_database_structure};


    println!("🚀 Starting Clock-It server...");

    clock_it::database::print_test_hash();
        
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;

    // Initialize database
    println!("🗄️ Initializing database...");
    let _pool = match init_db_pool().await {
    Ok(pool) => {
        if let Err(e) = test_db_connection(&pool).await {
            eprintln!("❌ Database connection test failed: {}", e);
            std::process::exit(1);
        }

        if let Err(e) = run_migrations(&pool).await {
            eprintln!("❌ Failed to run database migrations: {}", e);
            std::process::exit(1);
        }

        if let Err(e) = test_database_structure(&pool).await {
            eprintln!("❌ Database structure test failed: {}", e);
            std::process::exit(1);
        }

        pool
    }
    Err(e) => {
        eprintln!("❌ Failed to initialize database: {}", e);
        std::process::exit(1);
    }
};

    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // Run our app with hyper
    log!("🌐 Server listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}