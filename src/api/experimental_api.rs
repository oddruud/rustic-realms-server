use tide::Request;
use tide::Response;
use tide::Server;

use crate::services::ai::ChatGPTService;
use crate::combat::Brawl;
use crate::conversation::Question;

pub struct ExperimentalAPI{}

impl ExperimentalAPI
{
    pub fn register_endpoints(server: &mut Server<()>)
    {
        //TEST if connection is there
        server.at("/").get(Self::ping);
        server.at("/talk").get(Self::talk);
        server.at("/fight").get(Self::fight);
    }

    pub async fn ping(mut req: Request<()>) -> tide::Result 
    {
        let body = req.body_string().await?;
        println!("{:?}", body);
        
        let mut res = Response::new(201);
        res.set_body(String::from("Hola!"));
        Ok(res)
    }

    pub async fn talk(mut req: Request<()>) -> tide::Result 
    {
        let question: Question = req.body_json().await?;

        let mut result = String::from("");

            match ChatGPTService::answer(&question).await{
                Ok(answer) => result = answer,
                Err(error) => result = String::from(error.to_string())    
            }

            let mut res = Response::new(201);
            res.set_body(String::from(format!("{}", result)));
        Ok(res)
    }

    pub async fn fight(mut req: Request<()>) -> tide::Result 
    {
        let mut brawl: Brawl = req.body_json().await?;
        println!("{:?}", brawl);

        let fight_result = brawl.test_brawl();
        let conditions = String::from("no conditions yet");
        let mut res = Response::new(201);

        res.set_body(String::from(format!("Fight result: {}. Conditions: {}", fight_result, conditions)));
        Ok(res)
    }

}
