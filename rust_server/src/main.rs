use std::{fs::File, io::BufReader};
use actix_files::Files;
use actix_files::NamedFile;
use actix_web::{middleware, web, App, HttpRequest, HttpServer,Result};
use rdev::{simulate, EventType, Key, SimulateError};
use log::debug;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};


static X_MIN: f64      = -13.7;
static X_MAX: f64      =  13.7;

static Y_MIN: f64      = -35.5;
static Y_MAX: f64      =  13.5;

static ACCEL_MIN : f64 = -25.0 ;
static ACCEL_MAX : f64 =  25.0 ;


// Send static files
async fn index(req: HttpRequest) -> Result<NamedFile> {
    debug!("{req:?}");
    Ok(NamedFile::open("../build/index.html")?)
}


async fn values(req: HttpRequest) ->  Result<String> {

    let v: Vec<f64> = req.query_string()
    .trim_start_matches("v=")
    .split(',').map(|v| v.parse().unwrap()).collect();
    let x         = v[0];
    let y         = v[1];
    let z         = v[2];

    let alpha     = v[3];
    let beta      = v[4];
    let gamma     = v[5];


    let jump      = v[6];
    let boost     = v[7];
    let z_default = v[8]; 

    let z_min: f64 = (z_default  - 17.0) % 360.0;
    let z_max: f64 = (z_default  + 17.0) % 360.0;
    ////println!("{}",format!("Values :\nx : {}\ny : {}\nz : {}\n",v[0],v[1],v[2]));
    send_event_keys_accel(x ,y,z,alpha,beta,gamma,z_min,z_max,jump,boost);

    Ok(format!("V"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let config = load_rustls_config();

    log::info!("starting HTTPS server at https://localhost:8443");

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            // register simple handler, handle all methods
            .route("/api", web::get().to(values))
            .service(web::resource("/index.html").to(index))
            .service(web::redirect("/", "/index.html"))
            .service(Files::new("/", "../build/"))
        
    })
    .bind_rustls("0.0.0.0:8443", config)?
    .run()
    .await
}





fn load_rustls_config() -> rustls::ServerConfig {
    // init server config builder with safe defaults
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    // load TLS key/cert files
    let cert_file = &mut BufReader::new(File::open("certificate.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("private_key.pem").unwrap());

    // convert files to key/cert objects
    let cert_chain = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();

    // exit if no keys could be parsed
    if keys.is_empty() {
        //println!("Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }

    config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}


fn send(event_type: &EventType) {
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            //println!("We could not send {:?}", event_type);
        }
    }
}


fn send_event_keys(x :f64,y:f64,z:f64,alpha:f64,beta:f64,gamma:f64,z_min:f64,z_max:f64,jump:f64,boost:f64){
        //Y
    if y > Y_MAX {
        send(&EventType::KeyPress(Key::KeyW));
        send(&EventType::KeyRelease(Key::KeyS)); 
        //println!("pressed W forward");
    }
    else if y < Y_MIN{
        send(&EventType::KeyPress(Key::KeyS));
        send(&EventType::KeyRelease(Key::KeyW));
        //println!("pressed S backward");

    }
    else{
        if beta > ACCEL_MAX{
            send(&EventType::KeyPress(Key::KeyS));
            send(&EventType::KeyRelease(Key::KeyW));
            //println!("pressed S ACCEL");

        }
        else if beta < ACCEL_MIN {
            
            send(&EventType::KeyPress(Key::KeyW));
            send(&EventType::KeyRelease(Key::KeyS)); 
            //println!("pressed W ACCEL");

        }
        else{
            send(&EventType::KeyRelease(Key::KeyW));
            send(&EventType::KeyRelease(Key::KeyS));   
            //println!("Released S and W");

        }

    }
//X
    if x > X_MAX {
        send(&EventType::KeyPress(Key::KeyD));
        send(&EventType::KeyRelease(Key::KeyA));
        //println!("pressed D right");

    }
    else if x < X_MIN{
        send(&EventType::KeyPress(Key::KeyA));
        send(&EventType::KeyRelease(Key::KeyD)); 
        //println!("pressed A left");

    }
    else{
        if alpha < ACCEL_MIN{
            send(&EventType::KeyPress(Key::KeyA));
            send(&EventType::KeyRelease(Key::KeyD)); 
            //println!("pressed A accel");
        }
        else if alpha > ACCEL_MAX {
            send(&EventType::KeyPress(Key::KeyD));
            send(&EventType::KeyRelease(Key::KeyA));
            //println!("pressed D accel");

        }
        else{
            send(&EventType::KeyRelease(Key::KeyA));
            send(&EventType::KeyRelease(Key::KeyD));
            //println!("release A+D");
   
        }
    }

    //Z
    if z > z_max {
        send(&EventType::KeyPress(Key::LeftArrow));
        send(&EventType::KeyRelease(Key::RightArrow));  
        //println!("pressed LEFT {} {}",z,z_max);


    }
    else if z < z_min{
        send(&EventType::KeyPress(Key::RightArrow));
        send(&EventType::KeyRelease(Key::LeftArrow));
        //println!("pressed RIGTH");


    }
    else{
        if gamma > ACCEL_MAX{
            send(&EventType::KeyPress(Key::RightArrow));
            send(&EventType::KeyRelease(Key::LeftArrow));
            //println!("pressed RIGTH ACCEL");


        }
        else if gamma < ACCEL_MIN {
            send(&EventType::KeyPress(Key::LeftArrow));
            send(&EventType::KeyRelease(Key::RightArrow));  
            //println!("pressed LEFT ACCEL");


        }
        else{
            send(&EventType::KeyRelease(Key::LeftArrow));
            send(&EventType::KeyRelease(Key::RightArrow));   
        }
    }

    // JUMP BOOST
    if jump == 1.0{
        send(&EventType::KeyPress(Key::Space));
    }
    else {
        send(&EventType::KeyRelease(Key::Space));
    }
    if boost == 1.0{
        send(&EventType::KeyPress(Key::UpArrow));
    }
    else {
        send(&EventType::KeyRelease(Key::UpArrow));
    }  
}



fn send_event_keys_accel(x :f64,y:f64,z:f64,alpha:f64,beta:f64,gamma:f64,z_min:f64,z_max:f64,jump:f64,boost:f64){
    //Y
    if  beta < ACCEL_MIN {
        send(&EventType::KeyPress(Key::KeyW));
        send(&EventType::KeyRelease(Key::KeyS)); 
        //println!("pressed W forward");
    }
    else if   beta > ACCEL_MAX{
        send(&EventType::KeyPress(Key::KeyS));
        send(&EventType::KeyRelease(Key::KeyW));
        //println!("pressed S backward");

    }
    else{
        if y < Y_MIN {
            send(&EventType::KeyPress(Key::KeyS));
            send(&EventType::KeyRelease(Key::KeyW));
            //println!("pressed S ACCEL");

        }
        else if y > Y_MAX {
            
            send(&EventType::KeyPress(Key::KeyW));
            send(&EventType::KeyRelease(Key::KeyS)); 
            //println!("pressed W ACCEL");

        }
        else{
            send(&EventType::KeyRelease(Key::KeyW));
            send(&EventType::KeyRelease(Key::KeyS));   
            //println!("Released S and W");

        }

    }
    //X
    if  alpha > ACCEL_MAX {
        send(&EventType::KeyPress(Key::KeyD));
        send(&EventType::KeyRelease(Key::KeyA));
        //println!("pressed D right");

    }
    else if   alpha < ACCEL_MIN {
        send(&EventType::KeyPress(Key::KeyA));
        send(&EventType::KeyRelease(Key::KeyD)); 
        //println!("pressed A left");

    }
    else{
        if x < X_MIN{
            send(&EventType::KeyPress(Key::KeyA));
            send(&EventType::KeyRelease(Key::KeyD)); 
            //println!("pressed A accel");
        }
        else if x > X_MAX{
            send(&EventType::KeyPress(Key::KeyD));
            send(&EventType::KeyRelease(Key::KeyA));
            //println!("pressed D accel");

        }
        else{
            send(&EventType::KeyRelease(Key::KeyA));
            send(&EventType::KeyRelease(Key::KeyD));
            //println!("release A+D");

        }
    }

    //Z
    if gamma < ACCEL_MIN {
        send(&EventType::KeyPress(Key::LeftArrow));
        send(&EventType::KeyRelease(Key::RightArrow));  
        //println!("pressed LEFT {} {}",z,z_max);


    }
    else if  gamma > ACCEL_MAX{
        send(&EventType::KeyPress(Key::RightArrow));
        send(&EventType::KeyRelease(Key::LeftArrow));
        //println!("pressed RIGTH");


    }
    else{
        if  z < z_min{
            send(&EventType::KeyPress(Key::RightArrow));
            send(&EventType::KeyRelease(Key::LeftArrow));
            //println!("pressed RIGTH ACCEL");


        }
        else if z > z_max  {
            send(&EventType::KeyPress(Key::LeftArrow));
            send(&EventType::KeyRelease(Key::RightArrow));  
            //println!("pressed LEFT ACCEL");


        }
        else{
            send(&EventType::KeyRelease(Key::LeftArrow));
            send(&EventType::KeyRelease(Key::RightArrow));   
        }
    }

    // JUMP BOOST
    if jump == 1.0{
        send(&EventType::KeyPress(Key::Space));
    }
    else {
        send(&EventType::KeyRelease(Key::Space));
    }
    if boost == 1.0{
        send(&EventType::KeyPress(Key::UpArrow));
    }
    else {
        send(&EventType::KeyRelease(Key::UpArrow));
    }  
}