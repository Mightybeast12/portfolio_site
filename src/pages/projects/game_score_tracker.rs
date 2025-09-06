use yew::prelude::*;

pub fn game_score_tracker_page() -> Html {
    html! {
        <div class="home-container">
            <div class="markdown-container">
                {game_score_tracker_markdown()}
                <div class="button-20-container">
                    <a href="https://github.com/Mightybeast12/game-score-tracker" target="_blank" class="button-link">
                        <div class="button-20">
                            <span class="button-20-text">{"VIEW ON GITHUB"}</span>
                            <span class="button-20-arrow"></span>
                        </div>
                    </a>
                </div>
            </div>
        </div>
    }
}

pub fn game_score_tracker_intro() -> Html {
    html! {
        <div>
            <h1>{"🏆 Dynamic Sports Score Tracker"}</h1>
            <p>{"A serverless sports score tracking application built with AWS Fargate, Lambda, DynamoDB, and API Gateway. Easily configurable for different sports."}</p>
            <ul class="feature-list">
                <li>{"Serverless backend with AWS Lambda and DynamoDB"}</li>
                <li>{"Real-time score tracking with proper tennis scoring"}</li>
                <li>{"Dynamic resource naming for multi-sport support"}</li>
                <li>{"One-command deployment with Terraform"}</li>
            </ul>
        </div>
    }
}

pub fn game_score_tracker_markdown() -> Html {
    html! {
        <div>
            <h1>{"🏆 Dynamic Sports Score Tracker"}</h1>

            <p>{"A serverless sports score tracking application built with AWS Fargate, Lambda, DynamoDB, and API Gateway. Easily configurable for different sports (tennis, football, basketball, etc.)."}</p>

            <h2>{"Architecture"}</h2>
            <ul>
                <li><strong>{"Frontend"}</strong>{": HTML/JS app hosted on Fargate behind ALB"}</li>
                <li><strong>{"Backend"}</strong>{": Lambda functions for game management and scoring"}</li>
                <li><strong>{"Database"}</strong>{": DynamoDB for storing game state and history"}</li>
                <li><strong>{"API"}</strong>{": API Gateway for REST endpoints"}</li>
                <li><strong>{"Dynamic Naming"}</strong>{": All resources use configurable app name prefix"}</li>
            </ul>

            <h2>{"Features"}</h2>
            <ul>
                <li>{"✅ Create new games with player names"}</li>
                <li>{"✅ Real-time score tracking with proper tennis scoring (0, 15, 30, 40)"}</li>
                <li>{"✅ Automatic game, set, and match progression"}</li>
                <li>{"✅ Best of 3 sets format"}</li>
                <li>{"✅ Winner detection and game completion"}</li>
                <li>{"✅ Game history viewing"}</li>
                <li>{"✅ Dynamic sport naming (tennis → football, etc.)"}</li>
            </ul>

            <h2>{"AWS Services Used"}</h2>
            <ul>
                <li>{"🏗️ VPC: Custom VPC with public/private subnets"}</li>
                <li>{"⚖️ ALB: Application Load Balancer for frontend access"}</li>
                <li>{"🐳 ECS: Fargate service running in private subnets"}</li>
                <li>{"⚡ Lambda: Functions for game logic (create, score, history)"}</li>
                <li>{"💾 DynamoDB: Game state storage with configurable table name"}</li>
                <li>{"🔌 API Gateway: REST API with CORS support"}</li>
                <li>{"📦 ECR: Container registry for frontend image"}</li>
            </ul>

            <h2>{"Key Highlights"}</h2>
            <ul>
                <li>{"🚀 Serverless backend with automatic scaling"}</li>
                <li>{"🔧 One-command deployment: "}<code>{"./deploy.sh"}</code></li>
                <li>{"🏷️ Dynamic resource naming for multi-sport support"}</li>
                <li>{"🔒 Proper network isolation with security groups"}</li>
                <li>{"💰 Cost-effective with pay-per-use Lambda functions"}</li>
                <li>{"📊 Real-time score updates and game progression"}</li>
            </ul>

            <h2>{"Deployment"}</h2>
            <p>{"Simple one-command deployment that:"}</p>
            <ul>
                <li>{"1. Builds and pushes Docker image to ECR"}</li>
                <li>{"2. Deploys all AWS infrastructure with Terraform"}</li>
                <li>{"3. Updates frontend with API Gateway URL"}</li>
                <li>{"4. Forces ECS service update"}</li>
            </ul>
        </div>
    }
}
