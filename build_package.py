import os
import shutil
import zipfile
import re

if __name__ == "__main__":
    # create dist folder
    if os.path.exists("dist"):
        shutil.rmtree("dist")

    os.makedirs("dist")

    # run rust build command
    os.system("cargo build --release --package eigeen-loader")

    # run xmake build command
    os.chdir("d3d11")
    os.system("xmake build")
    os.chdir("..")

    # copy plugin files to dist folder
    shutil.copy("target/release/eigeen_loader.dll", "dist/eigeen_loader.dll")
    shutil.copy("d3d11/build/windows/x64/release/d3d11.dll", "dist/d3d11.dll")

    # create resource folder
    os.makedirs("dist/eigeen_loader/address")
    os.makedirs("dist/eigeen_loader/plugins")

    # copy address records json file to resource folder
    shutil.copy(
        "eigeen-loader/src/address/address_records.json",
        "dist/eigeen_loader/address/address_records.json",
    )

    # copy plugins to resource folder
    pass

    # get version from Cargo.toml
    version = ""
    with open("Cargo.toml", "r") as f:
        for line in f:
            if line.startswith("version"):
                results = re.findall(r'version = "(\d+\.\d+\.\d+)"', line)
                if len(results) > 0:
                    version = results[0]

    # create zip file
    archive = zipfile.ZipFile(f"dist/eigeen_loader_{version}.zip", "w", zipfile.ZIP_DEFLATED)
    
    archive.write("dist/eigeen_loader.dll", "eigeen_loader.dll")
    archive.write("dist/d3d11.dll", "d3d11.dll")
    archive.write(
        "dist/eigeen_loader/address/address_records.json",
        "eigeen_loader/address/address_records.json",
    )
    archive.mkdir("eigeen_loader/plugins")

    archive.close()