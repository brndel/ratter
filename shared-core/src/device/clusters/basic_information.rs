use crate::{device::clusters::define_cluster_macro::define_cluster, id::{ProductId, VendorId}};




define_cluster!(
    struct BasicInformation, enum BasicInformationChange, basic_information {
        product_id: ProductId => PRODUCT_ID as SetProductId { decode_product_id },
        product_name: String => PRODUCT_NAME as SetProductName { decode_product_name },
        vendor_id: VendorId => VENDOR_ID as SetVendorId { decode_vendor_id },
        vendor_name: String => VENDOR_NAME as SetVendorName { decode_vendor_name },
        software_version: u32 => SOFTWARE_VERSION as SetSoftwareVersion { decode_software_version },
        software_version_string: String => SOFTWARE_VERSION_STRING as SetSoftwareVersionString { decode_software_version_string }
    }
);