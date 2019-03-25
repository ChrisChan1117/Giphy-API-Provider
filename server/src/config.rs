use base64;
use envy;
use serde_derive::Deserialize;

// TODO: move these to env.
const GIPHY_API_KEY: &str = "hYncGxluLVmXPdUaber3iaF2F3EozYab";
const PRIVATE_KEY: &str = "LS0tLS1CRUdJTiBSU0EgUFJJVkFURSBLRVktLS0tLQpNSUlFb3dJQkFBS0NBUUVBclVXUDRUSVFSYmQ0MEFqdUtabDd5KzJLNThlSS9iMVFJa1p4RE9pSEhIUjFRU3kyCkZUb2gyY2gydkl3V25GWCtRQUY2M2lUMTV5U2VBQzRCUzIvNjBNTzdaRFJRWllIVXZnZjhIRndHTXM0YWpseXQKZmprVFozWko5cmVESVE0UEtqNGd0Nzl3d0pYTzVKMHdRTVROc1VaMmJkbkhQKzNnRXJZay9WSit2YnNYUnh1UQp0elU3NEJjVERQV3FWNHZoSTdOVzZFQ0JnNjVacGMwSk12Mm1BbWNjdUJVMDhUT3VMT1U0V05zbTlRa0ZkcTYwCnErdmhYMmJmaCtwaHdVWXhhaDVXdFFLU2wrODE0RDE1WWZiWk54R05BQjNFK0c0aFQwTG5MZ2NVTncxOVlWNGcKQldLNVZvOW0vRlN0NUU0cmhGRmNiZkVLdjllaDgwOTJMK2N1MVFJREFRQUJBb0lCQUdTaytpelJPcGgwT0syRgo4bWloYVVJTHFWcDJFVUZwbkVrOFoyQWJGSmZYb1dBSkMzNks5S3llZnpHSG9YNlBpSTQvRGRKSTM3aXlFVUNECmVHY01GVUQ3NERZVkdQbk5EdmlPaE1jb1FNVno2QUwzSENNc0hmUkR4alNUWll4bXNUSlZRZ1haSWpJTEVsREMKYVF2R0JNQks4SEZnUlJjQ0lkeGNobUhLY0NuZHRMZ1lzR21TdElWSU1HWHlJeTk4ZXFCRGtXT1JHS0xRQnAzKwpuVEtWQm82dWs0c1llZ0lmNnJkanhPSmhvSUp0bFovZTFVbmVEYzhaZzRBNTkwcDNkbDRCQmJXcThWNFQwbmtWCk9ZZXlKcHdMNWZXdHpoYlRocHhJeWFIVkw0QmRjRG5kNG03bm9TYmNQVXJ2K1VFdmhSQlZxdkE1QWd0L3VKeW4KWmtXZHdpVUNnWUVBMVR4WGRyd250TEtGeHhxMnA1a1dEVVlxS1A5NmFNNmJaRXpQaHFZN3pEOHdScldHa2JqZApGa2k2dWp6KzFTcXk4cUhuTVQ3SUh3Y29STFBSK0hnUU80ZHI0a1Z6S0VVc0ZrdGZ2eWg2U0VlZDJQZ052RTRCCmhsRzhoZDRoTkNGUTl2OStNTE1VbUpoRnh3VGtYSFNZS3RyN2VlVE84UjJqelc2Q24yRmZiRk1DZ1lFQTBBVngKbmJwa0xjaHBqQ1JlM2JmLzg0ajVmdUpINjhXR2tKRWRqWi9qYUVtQktMUGVOUGQwd3o3RXVMa0lsWVlvWHNYNwpOc2lmdzJtMndZMStTcE9vZEZpQ2JVb0pnZGNtSTJwVzM3aytSdnh3OCtOWWhpWlNPOXduSW4xbWdqTk9mSnZlCnlpSTl4YmxjdmpOdU4vZkFqeU5TWVIwMFM4L2hVRmZuY1pHWVV6Y0NnWUFNbUU3V1BaRGUxSGlnWlF5SzQ3ancKdWNjOXVyRmhSV3JhUnNzdk9keG9XUWNoOWhEbnhKS1NnV00rSXVJdWR5SXd3SzFtTkQ0Znp3MXEyWEE2aStUeApqek1wSDA4SS91VDJ3Q2I4cEloc2dvWnBUV1dLVE5VOE9ORzQyb0Q5a2tZMUFLeFVkZzl1WC9nbkpEdlUxakl5CnZkS0N4YWdQaDkrbi83SUpkZmZyN3dLQmdRREdHaWtWR0lpR3dURWk4UkFXL2orVjRZQVl3YzNVVG92VDZSN0gKM05ob1hIZlFGMXNwL2U4SWNTSk8rSWVnUGQ3OFFuYjJ1U0RNckVRTEY0Q0o3UXlqb0JhMWxhaHdLa0I3d2ExeAo2YXZSVmw4TEpCc054ampTSU5wMEVoQXBOa3NzaUJRblUyeVllK3BDaUNBUzFMSmduamE1bldKTVFIQkdYekJlCkMxV1JDUUtCZ0hqRjVGRjFvSlR3WjFmUDlKLzA3TVI1UHdnWUdRa0NxZk50T2xESWFNTHA0a3ByZXZuRkZpQ3EKTXB2K3ZMSVl6SlNma2RuK1VOMXIvVTY5dmpxdXpwbHA3NnJUTjlmZFZOVDN2QThJMWNaOXB3N3B3TjI1N1EySQorbmFPeVFMMHFuWEdnYnFOQUdmQ1I2L1dMWmhYa0FqbWpXYnlzdzE1V0pranRjaTlmSlE0Ci0tLS0tRU5EIFJTQSBQUklWQVRFIEtFWS0tLS0tCg==";
const PUBLIC_KEY: &str = "LS0tLS1CRUdJTiBQVUJMSUMgS0VZLS0tLS0KTUlJQklqQU5CZ2txaGtpRzl3MEJBUUVGQUFPQ0FROEFNSUlCQ2dLQ0FRRUFyVVdQNFRJUVJiZDQwQWp1S1psNwp5KzJLNThlSS9iMVFJa1p4RE9pSEhIUjFRU3kyRlRvaDJjaDJ2SXdXbkZYK1FBRjYzaVQxNXlTZUFDNEJTMi82CjBNTzdaRFJRWllIVXZnZjhIRndHTXM0YWpseXRmamtUWjNaSjlyZURJUTRQS2o0Z3Q3OXd3SlhPNUowd1FNVE4Kc1VaMmJkbkhQKzNnRXJZay9WSit2YnNYUnh1UXR6VTc0QmNURFBXcVY0dmhJN05XNkVDQmc2NVpwYzBKTXYybQpBbWNjdUJVMDhUT3VMT1U0V05zbTlRa0ZkcTYwcSt2aFgyYmZoK3Bod1VZeGFoNVd0UUtTbCs4MTREMTVZZmJaCk54R05BQjNFK0c0aFQwTG5MZ2NVTncxOVlWNGdCV0s1Vm85bS9GU3Q1RTRyaEZGY2JmRUt2OWVoODA5MkwrY3UKMVFJREFRQUIKLS0tLS1FTkQgUFVCTElDIEtFWS0tLS0tCg==";

/// Config defines this API's runtime configuration.
#[derive(Deserialize)]
pub struct Config {
    // Database Config //
    pub backend_connection_string: String,
    pub backend_database: String,

    // Auth Config //
    pub idp_private_key: String,
    pub idp_public_key: String,
    #[serde(skip)]
    pub raw_idp_private_key: String,
    #[serde(skip)]
    pub raw_idp_public_key: String,

    // Giphy API Config //
    pub giphy_api_key: String,
}

impl Config {
    /// Create a new config instance populated from the runtime environment.
    pub fn new() -> Config {
        // TODO: move these to env.
        std::env::set_var("GIPHY_API_KEY", GIPHY_API_KEY);
        std::env::set_var("IDP_PRIVATE_KEY", PRIVATE_KEY);
        std::env::set_var("IDP_PUBLIC_KEY", PUBLIC_KEY);

        // Deserialize config from environment.
        let mut config = match envy::from_env::<Config>() {
            Ok(config) => config,
            Err(err) => panic!("{:#?}", err),
        };

        // Decode public & private keys.
        config.raw_idp_private_key = String::from_utf8(
            base64::decode(&config.idp_private_key).expect("Expected private key to be base64 encoded.")
        ).expect("Expected private key to be valid UTF8.");
        config.raw_idp_public_key = String::from_utf8(
            base64::decode(&config.idp_public_key).expect("Expected public key to be base64 encoded.")
        ).expect("Expected public key to be valid UTF8.");

        // All is good. No panic. Return config instance.
        config
    }
}
