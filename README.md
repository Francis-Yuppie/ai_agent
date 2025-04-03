# CipherMind - AI-Powered GitHub Code Analyzer for ICP

CipherMind is an Internet Computer (ICP) canister that leverages AI, specifically ICP LLM , to scan and monitor deployed canisters on ICP alos analyze GitHub repositories for security vulnerabilities and code quality issues. It provides developers with automated code scanning directly on the ICP, enhancing the security and reliability of their projects.

## Features

* **GitHub Repository Analysis:** Input a GitHub repository URL, and CipherMind will fetch and analyze the code.
* **AI-Powered Vulnerability Detection:** Utilizes ChatGPT to identify potential security vulnerabilities in the code.
* **Rust Code Parsing:** Parses Rust code using the `syn` crate for accurate analysis.
* **Automated Scanning:** Provides a foundation for automated scanning and reporting.
* **ICP Native:** Runs directly on the Internet Computer, ensuring secure and decentralized analysis.

## Getting Started

### Prerequisites

* DFX (Internet Computer SDK) installed.
* Node.js and npm installed (for frontend, if applicable).
* A GitHub repository URL to analyze.
* An OpenAI API key.

### Deployment

1.  **Clone the Repository:**
    ```bash
    git clone [your-CipherMind-repository-url]
    cd CipherMind/
    ```

2.  **Start the Local Replica (for testing):**
    ```bash
    dfx start --background
    ```

3.  **Deploy the Canister:**
    ```bash
    dfx deploy
    ```

    * This command deploys your canister to the local replica and generates the Candid interface.
    * Make sure you have added your OpenAI API key in the code.
    * Also make sure that you have added your chat gpt endpoint in the code.

4.  **Interact with the Canister:**

    * You can interact with the canister using `dfx canister call` or through a frontend interface.
    * To analyze a GitHub repository, call the `analyze_repo` function:

    ```bash
    dfx canister call CipherMind_backend analyze_repo '(record { repo_url = "[https://github.com/your-username/your-repo](https://www.google.com/search?q=https://github.com/your-username/your-repo)" })'
    ```

5.  **View the Analysis Report:**

    * The analysis report will be returned by the `analyze_repo` function.
    * You can integrate this report into a frontend application for a better user experience.

### Frontend Development (Optional)

1.  **Start the Frontend Server:**
    ```bash
    npm start
    ```

    * This will start a development server at `http://localhost:8080`, proxying API requests to the replica at port 4943.

### Candid Interface Generation

If you make changes to the canister's Candid interface, regenerate it with:

```bash
npm run generate