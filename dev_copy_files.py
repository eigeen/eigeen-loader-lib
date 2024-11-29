import shutil
import os

os.system("cargo build --release --package eigeen-loader --features log_trace")

shutil.copy(
    "target/release/eigeen_loader.dll",
    "C:/Program Files (x86)/Steam/steamapps/common/Monster Hunter World/eigeen_loader.dll",
)
shutil.copy(
    "eigeen-loader/src/address/address_records.json",
    "C:/Program Files (x86)/Steam/steamapps/common/Monster Hunter World/eigeen_loader/address/address_records.json",
)
