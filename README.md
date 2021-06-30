# Enclave_POC

Credit must go to https://github.com/apache/incubator-teaclave-sgx-sdk from which this repo was forked. It is based off the Rust Intel SGX SDK. 

# Install (Mac)

This installation is to run the enclave part of my Thesis POC locally. You can find the nodejs server and web app with installation insutrctions here - https://github.com/Astewart1510/WebApp_POC. 

To run this enclave I would suggest using VS Code and Docker. To run out of a docker container so please make sure you have the latest Docker installed on your laptop. For instructions please visit the docker homepage - https://docs.docker.com/docker-for-mac/install/. Once docker is installed and VS code is open with the docker extension installed, please follow the instructions below. 

1. Clone or download this repo.
2. Go to https://github.com/apache/incubator-teaclave-sgx-sdk#use-simulation-mode-for-non-sgx-enabled-machine-includes-macos and follow the installation steps provided by this section. To summarise, after you have cloned and downloaded the entire incubator-teaclave-sgx-sdk repo your actions should be as follows:
    *  Navigate to the samplecode folder (eg. `cd downloads/incubator-teaclave-sgx-sdk-master/samplecode/`) and copy the POC_Enclave repo into the samplecode folder.
    * Download the sgx docker image - `docker pull baiduxlab/sgx-rust`
    * Run the docker image with the incubator-teaclave-sgx-sdk-master repo -  `docker run -v /your/path/to/rust-sgx:/root/sgx -ti baiduxlab/sgx-rust`
    * Open visual studio code and you should see a running docker container under the docker tab - [![Screenshot-2021-06-30-at-17-14-15.png](https://i.postimg.cc/v86f5zmh/Screenshot-2021-06-30-at-17-14-15.png)](https://postimg.cc/t7psjFWn)
3. Check that you are able to run the samplecode/helloworld app from within Step 1. If successful then close the docker container and clone this repo and and copy the files into the samplecode folder contained within the Rust Intel SGX SDK cloned repo from Step 1. The path should be incubator-teaclave-sgx-sdk/samplecode/Enclave_POC. 
4. Run the docker image exactly like in Step 1 except instead of running the helloworld app run the Enclave_POC app. 
5. The app should initiate a server on our local machine at the address `http://localhost:9000` running off the rocket framework. 
6. Welldone! The enclave should be running and listening on port 9000 for the webapp. To start the web app please navigate to this page. 
