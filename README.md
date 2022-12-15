# Freelance Escrow payment protocol 

# 📝 About
- The freelance protocol is a protocol built on decentralized and open systems such as blockchain and decentralized storage. The freelance protocol aims to be used in escrow payments, decentralized profiling, building freelance platforms, and many other possible use cases. It can be used by anyone whether it’s a designer, writer, developer, or other professional.
It will help with,


## 🚀 It will help with:
 - Securing the payment,
 - Eliminate trust and third parties,
 - Bring transparency and authenticity



## 🚀 How it works:
The Protocol basically secures funds for a project between two individuals (A client and a freelancer) and helps to release the funds to the freelancer in batches depending on the number of milestones achieved. It also allow a third individual to be an observer in the project and serve as a judge in case of a Dispute

# 1
- Client initializes a project with the following information
  -  Client Public Key
  -  Freelancer Public Key,
  -  An observers Public Key(which would act as a judge in the case of a conflict),
  -  A multisig wallet Account with three owners (client, Freelancer and Observer)
  -  A project Info Account
  -  An Escrow PDA to hold funds

# 2
- Client and the Freelancer creates a transaction and signs it to begin the project officially. At this point, the client cant withdraw the funds

# 3
- Client and Freelancer work off-chain and mark the current milestone as completed when job is satisfactory

# 4
- If dispute arises, the observer gets involved and votes in favour of who was right (more conditions for this to be present in V2)

## Multisig Rules
- owners: 3
- threshold: 2



## 🔥 How to test

### Prerequisites

- <a href="https://docs.solana.com/cli/install-solana-cli-tools">Solana</a>

### Installation

- Fork the Repository

```
   $ git clone https://github.com/IMEF-FEMI/freelance_payment_protocol
   $ cd Sol-Loan-a-NFT 
   $ git remote add upstream https://github.com/IMEF-FEMI/freelance_payment_protocol
   $ yarn install
   $ yarn run build:program
   $ anchor test
```


### feedbacks would be greatly appreciated

For more information check [our slide](https://docs.google.com/presentation/d/1N-VkDUp7tNtKPjP0KvRN1islXi8XrY3rD3FqR7_yUvc/edit#slide=id.g1b9f938b582_0_2) 