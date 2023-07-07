# Freelance Escrow Payment Protocol

## About

The Freelance Escrow Payment Protocol is a decentralized and open system built on blockchain and decentralized storage. It provides a secure and transparent platform for escrow payments in various freelance scenarios. Whether you are a designer, writer, developer, or any other professional, this protocol can be used to ensure secure transactions, eliminate the need for trust in third parties, and bring transparency and authenticity to the freelance industry.

## How It Works

The protocol facilitates the secure handling of funds for a freelance project between a client and a freelancer. It also allows for the involvement of an observer who can act as a judge in case of disputes. Here's a step-by-step breakdown of how the protocol works:

1. **Client Initialization**
   - The client initiates a project by providing the following information:
     - Client Public Key
     - Freelancer Public Key
     - Observer's Public Key (to serve as a judge in case of conflicts)
     - Multisig Wallet Account with three owners (client, freelancer, and observer)
     - Project Info Account
     - Escrow PDA (Programmable Deposit Address) to hold funds

2. **Project Start**
   - The client and freelancer create and sign a transaction to officially start the project. At this point, the client cannot withdraw the funds.

3. **Milestone Completion**
   - The client and freelancer work off-chain and mark each milestone as completed when the job is satisfactory.

4. **Dispute Resolution**
   - If a dispute arises, the observer gets involved and votes in favor of the party they believe is right (more conditions for this will be added in V2).

## Multisig Rules
- Owners: 3
- Threshold: 2

## How to Test

### Prerequisites

- Install Solana and Rust. Instructions can be found in the following links:
  - Solana: [Installation Guide](https://docs.solana.com/cli/install-solana-cli-tools)
  - Rust: [Installation Guide](https://www.rust-lang.org/tools/install)

### Minor Notes

Before proceeding with the installation, it's important to understand the available instructions provided by the protocol. Here are some notable functions and their purposes:

- `initialize_project(ctx: Context<InitializeProjectContext>, total_project_funds: u64, milestones: u8)`: This function is used to initialize a project by specifying the total funds for the project and the number of milestones.

- `start_project(ctx: Context<MultisigAuth>)`: This function is used to start the project officially. It creates and signs a transaction to begin the project.

- `mark_current_milestone_completed(ctx: Context<MultisigAuth>)`: This function allows the client and freelancer to mark the current milestone as completed when the job is satisfactory.

- `withdraw_milestone_funds(ctx: Context<WithdrawMilestoneFundsContext>)`: This function is used to withdraw the funds associated with a completed milestone.

- `stop_project(ctx: Context<MultisigAuth>)`: This function is used to stop the project. It can be called by any of the involved parties to halt the project.

- `cancel_project(ctx: Context<StopProjectContext>)`: This function is used to cancel the project entirely. It can be called by the client to cancel the project and retrieve the remaining funds.

### Installation

Here are the step-by-step installation instructions for newbies to follow:

1. **Fork the Repository**
   - Open your terminal or command prompt.
   - Execute the following command to clone the repository:

     ```
     $ git clone https://github.com/IMEF-FEMI/freelance_payment_protocol
     ```

   - Navigate to the cloned directory:

     ```
     $ cd freelance_payment_protocol
     ```

2. **Set Up Rust**
   - Rust is a programming language used for the protocol. Follow the instructions below to install Rust:
     - Open your web browser and visit the Rust installation page: [Rust Installation Guide](https://www.rust-lang.org/tools/install).
     - Follow the guide provided on the page to install Rust on your system.

3. **Set Up Solana**
   - Solana is the blockchain platform required for running the protocol. Follow the instructions below to install Solana:
     - Open your web browser and visit the Solana installation page: [Solana Installation Guide](https://docs.solana.com/cli/install-solana-cli-tools).
     - Follow the guide provided on the page to install Solana on your system.

4. **Build and Test the Protocol**
   - Once you have Solana and Rust installed, proceed with the following commands:
     - Install the required dependencies:

       ```
       $ yarn install
       ```

     - Build the protocol program:

       ```
       $ yarn run build:program
       ```

     - Run the protocol tests:

       ```
       $ anchor test
       ```

   - Make sure all the tests pass without errors.

Congratulations! You have successfully installed the Freelance Escrow Payment Protocol. If you encounter any issues during the installation process, refer to the documentation or seek assistance from the protocol's support channels.

### Feedback

We greatly appreciate any feedback you have. Please feel free to provide your suggestions and improvements for the protocol.

