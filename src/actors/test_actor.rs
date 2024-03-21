use crate::actors::messages::TestActorMessage;
use anyhow::Result;
use nostr_sdk::prelude::Event;
use ractor::{Actor, ActorProcessingErr, ActorRef};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct TestActor;

pub struct TestActorState {
    pub published_messages: Arc<Mutex<Vec<Event>>>,
}

#[ractor::async_trait]
impl Actor for TestActor {
    type Msg = TestActorMessage;
    type State = TestActorState;
    type Arguments = Arc<Mutex<Vec<Event>>>;

    async fn pre_start(
        &self,
        _: ActorRef<Self::Msg>,
        published_messages: Arc<Mutex<Vec<Event>>>,
    ) -> Result<Self::State, ActorProcessingErr> {
        let state = TestActorState { published_messages };

        Ok(state)
    }

    async fn handle(
        &self,
        _: ActorRef<Self::Msg>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            TestActorMessage::EventHappened(event) => {
                state.published_messages.lock().await.push(event);
            }
        }

        Ok(())
    }
}
