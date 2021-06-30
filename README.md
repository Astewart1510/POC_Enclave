# Enclave_POC

Credit must go to https://github.com/apache/incubator-teaclave-sgx-sdk from which this repo was forked. It is based off the Rust Intel SGX SDK. 

This installation details the installation, setup and running of the enclave part of my Thesis POC locally. You can find the nodejs server and web app with installation insutrctions here - https://github.com/Astewart1510/WebApp_POC. 

# Setup and Install

This setup was done on a Macbook Pro 2015. For Windows, the instructions should be similar and the issues with the enclave will be solved the same, however please see windows documentation for VS Code and Docker setup and terminal syntax. To run this enclave I would suggest using VS Code and Docker. To run out of a docker container so please make sure you have the latest Docker installed on your laptop. For instructions please visit the docker homepage - https://docs.docker.com/docker-for-mac/install/. Once docker is installed and VS code is open with the docker extension installed and enabled, please follow the instructions below. 

1. Clone or download this repo.
2. Go to https://github.com/apache/incubator-teaclave-sgx-sdk#use-simulation-mode-for-non-sgx-enabled-machine-includes-macos and follow the installation steps provided by this section OR to summarise, after you have cloned and downloaded the entire incubator-teaclave-sgx-sdk repo your actions should be as follows:
    *  Navigate to the samplecode folder (eg. `cd downloads/incubator-teaclave-sgx-sdk-master/samplecode/`) and copy the POC_Enclave repo into the samplecode folder.
    * Download the sgx docker image - `docker pull baiduxlab/sgx-rust`
    * Run the docker image with the incubator-teaclave-sgx-sdk-master repo -  `docker run -v /your/path/to/rust-sgx:/root/sgx -ti baiduxlab/sgx-rust`
    * Open visual studio code and you should see a running docker container under the docker tab. Right click this docker container and select "Attach Visual Studio Code". See Screenshot.
    [![Screenshot-2021-06-30-at-17-14-15.png](https://i.postimg.cc/SQWjytsB/Screenshot-2021-06-30-at-17-14-15.png)](https://postimg.cc/NLjQDDRD)
    * Another VS window should open and right click the POC_Enclave-master file to open it in an Integrated Terminal. See screenshot.
    [![Screenshot-2021-06-30-at-17-19-44.png](https://i.postimg.cc/cJ9ktmJZ/Screenshot-2021-06-30-at-17-19-44.png)](https://postimg.cc/zLhF9TWP)
    * The setup is complete.
3.  Now to install the dependencies run the following command in the terminal window above (root@487c758960cf:~/sgx/samplecode/POC_Enclave-master# ) - `SGX_MODE=SW make`. This initiates the enclave to run in simulation mode for demonstration purposes. 
4. This part will take some time depending on speed of internet and processing power of laptop. I ran it off a 16GM RAM Macbook Pro 2015 wih a 100Mb/s line and it took 10 minutes. 
5. The latest Intel-SGX-Rust-SDK version is geared to only run on Rust nightly-10-25 and not on the latest nightly-06-27. Therefore, two failures will arise and following fixes need to be done manually. I will outline the process below, the process is the same both.
6. First error: `error[E0658]: use of unstable library feature 'unsafe_cell_get_mut'`
   * [![Compiling-async-trait-v0-1-49.png](https://i.postimg.cc/NfmvW5sY/Compiling-async-trait-v0-1-49.png)](https://postimg.cc/WhNyZpqf)
   * Hover over this line and hold Command + click to open the code - [![Compiling-unchecked-index-v0-2-2.png](https://i.postimg.cc/028Cq4xQ/Compiling-unchecked-index-v0-2-2.png)](https://postimg.cc/kDpSQf23)
   * Once the code is open hover over the bredcrmb trail at the top of the screen and select the lib.rs file. See screenshot. 
   [![X-Getting-Started.png](https://i.postimg.cc/j5MSnCWB/X-Getting-Started.png)](https://postimg.cc/47hgrfQ1)
   * Copy or type the text from the terminal and paste in the lib.rs file. As shown in screenshots. 
   [![help-add-feature-unsafe-cell-get-mut.png](https://i.postimg.cc/8zvddgmZ/help-add-feature-unsafe-cell-get-mut.png)](https://postimg.cc/bZz28MMn)
   [![samplecode-Container-baiduxlabsgx-rust-silly-fermi.png](https://i.postimg.cc/6p7KYY8j/samplecode-Container-baiduxlabsgx-rust-silly-fermi.png)](https://postimg.cc/75yjL1M0)
   * Save the lib.rs file and run `SGX_MODE=SW make` in the terminal again. 
7. Second error:  `error[E0658]: use of unstable library feature 'bool_to_option'`
   * Solved the same way as above. 
   * Hover over the path and open code. 
   * Once open hover over the breadcrumb trail and select the lib.rs file. 
   * Copy over or type the text in the lib.rs file. See screenshot. 
   [![samplecode-Container-baiduxlabsgx-rust-silly-fermi.png](https://i.postimg.cc/KYP8DJVX/samplecode-Container-baiduxlabsgx-rust-silly-fermi.png)](https://postimg.cc/3ywhKCY9)
   * Save the lib.rs file and run `SGX_MODE=SW make` in the terminal again. 
9. The build process should complete. This process will take a while. 
10. Once complete run this command in terminal - `cd bin`
11. Then to run the app - `./app`
12. The app should run and initiate a server on our local machine at the address `http://localhost:9000` running off the rocket framework. Please make sure that it is running off port 9000 and not 9001. See screenshot.
   [![Compiling-serde-cbor-V0-11-1-httpsgithub-commesalock-linuxcbor-sgx-6421105d.png](https://i.postimg.cc/tgcGPPR0/Compiling-serde-cbor-V0-11-1-httpsgithub-commesalock-linuxcbor-sgx-6421105d.png)](https://postimg.cc/06741zJZ)
15. Welldone! The enclave should be running and listening on port 9000 for the Web App. 
