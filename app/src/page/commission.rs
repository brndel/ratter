use dioxus::prelude::*;
use shared_core::{asset::device::DeviceAssetConfig, device::DeviceCommissionMode};

use crate::MatterManagerExt;
use crate::component::action_status::ActionStatus;
use crate::component::{
    device_asset_config::DeviceAssetConfigView,
    tab_bar::{TabBar, TabBarItem},
};

#[component]
pub fn CommissionForm() -> Element {
    let mut pairing_code = use_signal(String::new);
    let device_config = use_signal(DeviceAssetConfig::default);
    let mut mode = use_signal(|| DeviceCommissionMode::SharedCode);

    let mut run_commission = use_action(move || async move {
        commission_device(pairing_code(), device_config().clone(), mode()).await
    });

    rsx! {
        div { class: "list",
            DeviceAssetConfigView { device: device_config }

            label { "Commission Mode" }
            TabBar { value: mode, on_select: move |value| mode.set(value),
                TabBarItem { value: DeviceCommissionMode::SharedCode, "Commission with share code" }
                TabBarItem { value: DeviceCommissionMode::Ble, "Commission new device with ble" }
            }

            label { r#for: "code", "Code" }
            input {
                id: "code",
                name: "code",
                placeholder: "XXXX-XXX-XXXX",
                value: "{pairing_code}",
                oninput: move |ev| {
                    pairing_code.set(ev.value());
                },
            }

            button {
                onclick: move |ev| async move {
                    ev.prevent_default();

                    run_commission.call();
                },
                "Commission"
            }

            ActionStatus { action: run_commission }
        }
    }
}

#[post("/api/commission", matter: MatterManagerExt)]
async fn commission_device(
    pairing_code: String,
    device: DeviceAssetConfig,
    mode: DeviceCommissionMode,
) -> Result<(), ServerFnError> {
    let pairing_code = pairing_code.replace("-", "");

    matter
        .commission_device(&pairing_code, device, mode)
        .await?;

    Ok(())
}
