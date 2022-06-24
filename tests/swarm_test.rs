#![type_length_limit = "2097152"]

use bollard::errors::Error;
use bollard::Docker;
use bollard::swarm::*;

use tokio::runtime::Runtime;

#[macro_use]
pub mod common;
use crate::common::*;

async fn swarm_test(docker: Docker) -> Result<(), Error> {
    let config = InitSwarmOptions {
        listen_addr: "0.0.0.0:2377",
        advertise_addr: "127.0.0.1",
    };
    let _ = &docker
        .init_swarm(config)
        .await?;

    let inspection_result = &docker
        .inspect_swarm()
        .await?;
    
    assert!(inspection_result.join_tokens.as_ref().unwrap().worker.as_ref().unwrap().len() > 0);

    let config = LeaveSwarmOptions {
        force: true,
    };
    let _ = &docker
        .leave_swarm(Some(config))
        .await?;
    Ok(())
}

async fn inspect_swarm_test(docker: Docker) -> Result<(), Error> {
    let config = InitSwarmOptions {
        listen_addr: "0.0.0.0:2377",
        advertise_addr: "127.0.0.1",
    };
    let _ = &docker
        .init_swarm(config)
        .await?;
    
    let result = &docker
        .inspect_swarm()
        .await?;
    print!("init {}", result.join_tokens.as_ref().unwrap().worker.as_ref().unwrap());
    Ok(())
}

async fn join_swarm_test(docker: Docker) -> Result<(), Error> {
    let config = JoinSwarmOptions {
        advertise_addr: "127.0.0.1",
        join_token: "SWMTKN-1-4ssgk9ow7an36mjc0oh7nj5z10c036igse5nlxusq9j28nbz86-dftyw9ad7qgnp8lm5riwgs2ibtest",
    };
    let _ = &docker
        .join_swarm(config)
        .await?;
    Ok(())
}

async fn leave_swarm_test(docker: Docker) -> Result<(), Error> {
    let config = LeaveSwarmOptions {
        force: true,
    };
    let _ = &docker
        .leave_swarm(Some(config))
        .await?;

    Ok(())
}

#[test]
fn integration_test_swarm() {
    connect_to_docker_and_run!(swarm_test);
}

#[test]
fn integration_test_inspect_swarm() {
    connect_to_docker_and_run!(inspect_swarm_test);
}

#[test]
fn integration_test_join_swarm() {
    connect_to_docker_and_run!(join_swarm_test);
}

#[test]
fn integration_test_leave_swarm() {
    connect_to_docker_and_run!(leave_swarm_test);
}
