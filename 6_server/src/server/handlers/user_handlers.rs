use axum::extract::{Path, State};
use axum::Json;

use crate::data::repositories::user_repository::UserRepository;
use crate::models::user::{CreateUser, User};
use crate::server::state::UserRepositoryProvider;

pub async fn create_user<S: UserRepositoryProvider>(
    State(state): State<S>,
    Json(body): Json<CreateUser>,
) -> axum::response::Result<Json<User>> {
    Ok(Json(state.user_repository().create_user(body)?))
}

pub async fn get_user<S: UserRepositoryProvider>(
    State(state): State<S>,
    Path(id): Path<uuid::Uuid>,
) -> axum::response::Result<Json<User>> {
    Ok(Json(state.user_repository().get_user(id)?))
}


#[cfg(test)]
mod test {
    use std::str::FromStr;
    use std::sync::Arc;

    use email_address::EmailAddress;
    use mockall::mock;

    use crate::data::data_errors::DataError;
    use crate::data::repositories::user_repository::UserRepository;
    use crate::models::user::Username;

    use super::*;

    mock! {
        UserRepo {}
        impl Clone for UserRepo {
            fn clone(&self) -> Self;
        }
        impl UserRepository for UserRepo {
            fn create_user(&self, create_user: CreateUser) -> Result<User, DataError>;
            fn get_user(&self, id: uuid::Uuid) -> Result<User, DataError>;
        }
    }

    #[derive(Clone)]
    struct Provider {
        repo: Arc<MockUserRepo>,
    }

    impl UserRepositoryProvider for Provider {
        fn user_repository(&self) -> Arc<impl UserRepository> {
            self.repo.clone()
        }
    }

    fn setup(f: fn(&mut MockUserRepo)) -> Provider {
        let mut mock = MockUserRepo::new();
        f(&mut mock);
        Provider {
            repo: Arc::new(mock),
        }
    }

    fn gen_test_user() -> User {
        User {
            id: uuid::Uuid::from_bytes([0; 16]),
            username: Username::new("test").unwrap(),
            email: EmailAddress::from_str("test@test.com").unwrap(),
        }
    }


    #[tokio::test]
    async fn test_create_user() {
        let state = setup(|mock: &mut MockUserRepo|
        {
            mock.UserRepository_expectations.create_user.expect().returning(|_| Ok(gen_test_user()));
        }
        );


        let user = CreateUser {
            name: Username::new("test").unwrap(),
            email: EmailAddress::from_str("test@test.com").unwrap(),
        };

        let response = create_user(State(state.clone()), Json(user.clone())).await.unwrap();
        assert_eq!(response.0.username, user.name);
        assert_eq!(response.0.email, user.email);
    }

    #[tokio::test]
    async fn test_get_user() {
        let state = setup(move |mock| {
            mock.UserRepository_expectations.get_user.expect().returning(|_| Ok(gen_test_user()));
        });

        let response = get_user(State(state.clone()), Path(uuid::Uuid::from_bytes([0; 16]))).await.unwrap();
        assert_eq!(response.0, gen_test_user());
    }
}