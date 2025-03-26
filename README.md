🚀 Sirius Funding – Deployment Guide

Follow these steps to build and deploy the Sirius Funding smart contract to the testnet.
📂 1. Navigate to the Contract Directory

cd contracts/main

⚙️ 2. Update the Makefile

Open the Makefile and update the SOURCE flag with your own contract source file.
📤 3. Deploy the Contract

Run the following command to build and deploy the contract to the testnet:

make deploy

🆔 4. Retrieve the Contract ID

After deployment, the terminal will output the Contract ID.
You will need this ID to interact with the deployed contract using its functions.