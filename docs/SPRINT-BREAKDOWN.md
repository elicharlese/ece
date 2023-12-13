# Sprint Breakdown for Cleanfolio Project

## Sprint Day 1: Environment Setup and Initial Planning

### Days = 1

- **Total Sprint Points:** 20

- [x] Set up the Next.js project with TypeScript.
- [x] Integrate TailwindCSS and Sass for styling.
- [x] Prepare Docker environment for containerization.
- [x] Initialize pnpm workspace.
- [x] Scaffold basic file structure (components, pages, utilities).
- [x] Create basic theming components for dark/light mode toggling & state management.

- [x] Develop a comprehensive feature roadmap
- [x] Outline smart contract requirements for NEAR Rust SDK.
- [x] Define data models and interfaces for wallet integration and crowd funding center.

<br></br>

## Sprint Days 2-3: Smart Contracts and Basic UI Layout

- **Total Sprint Points:** 55

### Days =  2

- [ ] Import React & Tailwind from Figma
- [ ] Set-up config and dark/light theme
- [ ] Begin coding smart contracts using NEAR Rust SDK.
- [ ] Develop test cases for smart contracts.
- [ ] Home Page layout.
- [ ] Marketplace skeleton interface.
- [ ] Stub out UI components for wallet functionality.
- [ ] Integrate Thirdweb for Web3 functionalities.
- [ ] Implement Wallet connection and display user's ERC20 token balance.
- [ ] Implement purchase/swap functions in smart contracts.
- [ ] Continue writing test cases and perform initial testing.
- [ ] Style Home Page and Marketplace with TailwindCSS and Sass.
- [ ] Add responsive design elements for better mobile support.

<br></br>

## Sprint Days 4: Crowdfunding Center and Application Shells

- **Total Sprint Points:** 25

### Days = 1

- [ ] Scaffold backend logic for crowdfunding mechanism.
- [ ] Design database schema for tracking contributions.
- [ ] Create shell applications within the platform (TxnTracker, SocketChat, etc.).
- [ ] Define routing and base components for each application.

<br></br>

## Sprint Days 5-8: Applications Development Start

- **Total Sprint Points:** 50

### Days = 3

- **TxnTracker**: (4 points)
- [ ] Implementation of transaction tracking logic.
- [ ] UI development for transaction history.

- **SocketChat**: (4 points)
- [ ] Set up WebSocket server.
- [ ] Implement basic chat interface.

- **NanoNode**: (4 points)
- [ ] Initial setup of microservice for node communication.
- [ ] Interface to visualize node status.

- **Bitcellular**: (4 points)
- [ ] Define cellular automata rules and interactions.
- [ ] Build visual representation for automata state.
- [ ] Incorporate contracts and tests from Bitcell

- **ThePublic**: (4 points)
- [ ] Initiate a feed or forum-like component.
- [ ] Determine moderation and posting features.
- [ ] Incorporate contracts and tests from ThePublic

### Days = 1

- **Applications Development Finish**: (15 points)
- [ ] Complete any remaining logic and interfaces for TxnTracker, SocketChat, NanoNode, Bitcellular, and ThePublic.
- [ ] Ensure all apps are integrated and functioning within the overall platform.

- **Integration and End-to-End Testing**: (10 points)
- [ ] Test integration of all components, applications, and smart contract interactions.
- [ ] Debug issues and refine user experience.

- **Documentation and Cleanup**: (5 points)
- [ ] Write developer and user documentation for the platform.
- [ ] Code cleanup and optimization.

<br></br>

## Sprint Day 9: Final Touches and Deployment

- **Total Sprint Points:** 20

### Days = 1

- [ ] Conduct thorough UAT with potential users.
- [ ] Gather feedback and make necessary adjustments.
- [ ] Perform security audits on smart contracts.
- [ ] Address identified vulnerabilities.
- [ ] Containerize final application with Docker for deployment.
- [ ] Set up CI/CD pipeline if required.
- [ ] Deploy application to production.
- [ ] Monitor initial launch for urgent issues and fix as needed.

## Sprint Days 10-13: Automated Workflow for Web3 Marketplace Content Generation

- **Total Sprint Points: 30**

### Days = 4

- [ ] Set up automation using Zapier to trigger weekly tasks.
- [ ] Configure ChatGPT integration with Zapier for new use cases generation.
- [ ] Develop scripts for automated content creation:
- [ ] Docker container images
- [ ] Python scripts for trading bots
- [ ] Rust/TypeScript/Solidity smart contracts

 Create a repository structure for storing generated content.
 Write documentation for the automated workflow process.
 Implement testing procedures for the generated content.
 Establish an AWS S3 bucket for hosting the content.
 Configure AWS IAM roles and policies for secure access management.
 Design a seamless upload process from Zapier to AWS S3.
 Integrate the content upload process with the marketplace on the Cleanfolio platform.
 Ensure proper versioning for the content items to keep track of updates.
 Develop marketing materials for the newly created marketplace items.
 Conduct a small-scale pilot run of the automated workflow.


## Marketplace Automation Workflow Description:

The newly introduced segment in the sprint is designed to create an efficient pipeline that automates the production of marketplace items, which are custom docker container images, python scripts for trading bots, and custom smart contracts written in Rust/TypeScript/Solidity. This process begins with an automated weekly trigger configured via Zapier, which prompts ChatGPT to generate a list of fresh use cases for each item category.

Once ChatGPT has provided the use cases, corresponding files and content are automatically generated based on templates and pre-defined logic. The created items are then deposited into a structured repository, reviewed for quality, and uploaded to the AWS S3 bucket designated for marketplace content hosting. Tight integration within the platform’s marketplace ensures users have immediate access to purchase these new offerings.

To maintain operational security and manageability, the necessary IAM roles and policies will be set on AWS to control access. All content items will carry version numbers to handle updates and revisions systematically. Marketing efforts will accompany the release of each item to ensure visibility and drive sales.

An initial test run is crucial to iron out any kinks in the workflow and to ensure that all components interact smoothly. The successful deployment of this workflow sets the stage for consistent and regular updates to the web3 marketplace, providing customers with valuable and cutting-edge products.