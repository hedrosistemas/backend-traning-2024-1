pub mod server {
    tonic::include_proto!("server");
}
use server::{
    io_t_data_services_server::{IoTDataServices, IoTDataServicesServer},
    ListIoTDataRequest, ListIoTDataResponse,
};
use std::error::Error;
use tonic::transport::Server;

struct IoTDataServicesImpl {
    //client: >>>>>>>>
}

impl IoTDataServicesImpl {
    pub fn new(//client: >>>>>>>>
    ) -> Self {
        //Self {client}
        Self {}
    }
}

#[tonic::async_trait]
impl IoTDataServices for IoTDataServicesImpl {
    async fn list_io_t_data(
        &self,
        _req: tonic::Request<ListIoTDataRequest>,
    ) -> Result<tonic::Response<ListIoTDataResponse>, tonic::Status> {
        //realizar a consulta no timestrem
        // converter o dado obtido no timestream para o tipo criado no protofile IoTData, e adicionar esses valores em um vetor
        // retornar o vetor

        //self.client.query().........
        //conversao do dado
        //
        Ok(tonic::Response::new(ListIoTDataResponse { data: vec![] }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "0.0.0.0:50051".parse()?;

    //fazemos a conexao com o timestrem, usando o sdk da aws na crate timestreamquery;
    // let client = .........

    let service = IoTDataServicesImpl::new(
        //client
    );

    println!("staging grpc server");

    Server::builder()
        .add_service(IoTDataServicesServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
