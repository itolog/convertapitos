use crate::common::types::api_types::AppResponse;
use crate::common::types::app_types::AppResult;
use salvo::prelude::*;

#[handler]
pub async fn get_types(res: &mut Response) -> AppResult<()> {
    res.render(Json(AppResponse::<String> {
        data: "OK!!".to_string(),
        error: None,
    }));

    Ok(())
}
