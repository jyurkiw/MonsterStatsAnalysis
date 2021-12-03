// This is a modified version of the example at:
// https://github.com/Byron/google-apis-rs/tree/main/gen/sheets4

extern crate google_sheets4 as sheets4;
extern crate hyper;
extern crate hyper_rustls;
extern crate yup_oauth2 as oauth2;
use sheets4::Error;
use sheets4::Sheets;

pub mod monster;

#[tokio::main]
pub async fn get_monster_data() -> std::vec::Vec<monster::Monster> {
    let mut monsters: std::vec::Vec::<monster::Monster> = vec![];

    let secret = oauth2::read_application_secret("client_secret_412356003066-o84j3oel9m889eaapittt9jubmtvj4e3.apps.googleusercontent.com.json")
        .await
        .expect("client secret could not be read");

    let auth = oauth2::InstalledFlowAuthenticator::builder(
        secret,
        yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    )
    .persist_tokens_to_disk("tokencache.json")
    .build()
    .await
    .unwrap();

    let hub = Sheets::new(
        hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()),
        auth,
    );

    let result = hub
        .spreadsheets()
        .values_get("1QEAdD6BUKWfmIRTAvuNGMQwCIlaHfbTsuxAhUTPDaSk", "A2:T798")
        .doit()
        .await;

    match result {
        Err(e) => match e {
            Error::HttpError(_)
            | Error::Io(_)
            | Error::MissingAPIKey
            | Error::MissingToken(_)
            | Error::Cancelled
            | Error::UploadSizeLimitExceeded(_, _)
            | Error::Failure(_)
            | Error::BadRequest(_)
            | Error::FieldClash(_)
            | Error::JsonDecodeError(_, _) => println!("{}", e),
        },
        Ok(res) => {
            let values = match res.1.values {
                Some(v) => v,
                None => panic!("No values found."),
            };

            for d in &values {
                let m = monster::new_from_drive(d);
                monsters.push(m);
            }
        },
    }

    monsters
}