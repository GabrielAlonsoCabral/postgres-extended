use dotenv::dotenv;
use neon::prelude::*;
use postgres::{Client, NoTls};

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

type PostgresExtendedClient = Client;

struct PostgresAuth {
    user: String,
    password: String,
    host: String,
    port: String,
    database: String,
}

struct PostgresExtended {
    client: PostgresExtendedClient,
}

impl PostgresExtended {
    fn connection(auth: PostgresAuth) -> PostgresExtended {
        let connection_string: String = format!(
            "postgresql://{user}:{password}@{host}:{port}/{db}",
            user = auth.user,
            password = auth.password,
            host = auth.host,
            port = auth.port,
            db = auth.database
        );

        PostgresExtended {
            client: Client::connect(&connection_string, NoTls)
                .expect("Failed to connect to the database"),
        }
    }
}

fn show(mut cx: FunctionContext) -> JsResult<JsNull> {
    dotenv().ok();

    let auth_object: Handle<JsObject> = cx.argument(0)?;
    let (user, password, host, port, database) = (
        get_string_value_from_js_object(&mut cx, auth_object, "user"),
        get_string_value_from_js_object(&mut cx, auth_object, "password"),
        get_string_value_from_js_object(&mut cx, auth_object, "host"),
        get_string_value_from_js_object(&mut cx, auth_object, "port"),
        get_string_value_from_js_object(&mut cx, auth_object, "database"),
    );

    let auth: PostgresAuth = PostgresAuth {
        user,
        password,
        host,
        port,
        database,
    };

    let postgres_connection: PostgresExtended = PostgresExtended::connection(auth);
    let mut postgres_client: PostgresExtendedClient = postgres_connection.client;

    let result = postgres_client.query("SELECT * FROM cities", &[]);
    match result {
        Ok(rows) => {
            for row in rows {
                let id: i32 = row.get("city_id");
                println!("id: {}", id);
            }
        }
        Err(e) => {
            eprintln!("Error querying the database: {:?}", e);
            // Handle the error as needed
        }
    }
    Ok(cx.null())
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("show", show)?;

    Ok(())
}

pub fn get_env(variable: &str) -> String {
    std::env::var(variable).unwrap_or_else(|_| {
        panic!("{} must be set.", variable);
    })
}

fn get_string_value_from_js_object(
    cx: &mut FunctionContext,
    object: Handle<JsObject>,
    key: &str,
) -> String {
    let prop: Handle<JsValue> = object.get(cx, key).unwrap();

    let value: String = prop.downcast::<JsString, _>(cx).unwrap().value(cx);

    return value;
}
