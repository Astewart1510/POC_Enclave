# Enclave_POC

Credit must go to https://github.com/apache/incubator-teaclave-sgx-sdk from which this repo was forked. It is based off the Rust Intel SGX SDK. 

# Install 

This installation is to run the enclave part of my Thesis POC locally.

1. Go to https://github.com/apache/incubator-teaclave-sgx-sdk#use-simulation-mode-for-non-sgx-enabled-machine-includes-macos and follow the installation steps provided by this section of the repo Rust Intel SGX SDK.
2. Check that you are able to run the samplecode/helloworld app from within Step 1. If successful then close the docker container and clone this repo and and copy the files into the samplecode folder contained within the Rust Intel SGX SDK cloned repo from Step 1. The path should be incubator-teaclave-sgx-sdk/samplecode/Enclave_POC. 
3. Run the docker image exactly like in Step 1 except instead of running the helloworld app run the Enclave_POC app. 
4. The app should initiate a server on our local machine at the address `http://localhost:9000` running off the rocket framework. 
5. Welldone! The enclave should be running and listening on port 9000 for the webapp. To start the web app please navigate to this page. 
