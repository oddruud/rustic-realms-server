
use tide::Request;
use tide::Response;
use tide::Body;
use tide::Server;

use crate::player::Player;

pub struct PlayerAdminAPI{}

impl PlayerAdminAPI {

    pub fn register_endpoints(server: &mut Server<()>)
    {
        server.at("/player/new").post(Self::register_player);
        
        
        //app.at("/login").post(login_player);
    }

    pub async fn register_player(mut req: Request<()>) -> tide::Result {
        let player: Player = req.body_json().await?;
        // let get a mut ref of our store ( hashMap )
        //let mut dinos = req.state().dinos.write().await;
        //dinos.insert(String::from(&dino.name), dino.clone());
        
        let mut res = Response::new(201);
        
        print!("Registering {}",player.name);

        res.set_body(Body::from_json(&player)?);
        Ok(res)
    }

    async fn login_player(mut req: Request<()>) -> tide::Result {
        let player: Player = req.body_json().await?;
        let mut res = Response::new(201);
        
        print!("Logging in {}", player.name);

        res.set_body(Body::from_json(&player)?);
        Ok(res)
    }

    
}