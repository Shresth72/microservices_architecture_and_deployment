# Microservices Architecture in node.js / rust (rocket.rs)

**Initial Implementation**
-

- Multiple services that communicate through message broker. (Apache Kafka / RabbitMQ)
- Nginx to act as reverse proxy for managing request APIs for each service.
- Containerized services and database via Docker for easy deployment on Elastic Beanstalk. (Working on GCE)
- Continuous Deployment environment using GitHub workflows for testing and production.
- Scaling services using EC2 and Nginx, for multiple instances and load balancing.

## Microservice Architecture
<img src="process_diagrams\micrservice_arch.png" alt="drawing" style="width:600px;"/>

## Communication within services
<img src="process_diagrams\message_broker_communication.png" alt="drawing" style="width:600px;"/>

<br/><br/>

# Project Documentation

**Continuous Deployment Setup**
-
- Set new branch rules
- Setup workflows for Continuous Deployment Setup complete for running all tests
  - Pull Request enabled
  - Node.js environment and actions
  - Configure all service directories
  - Install dependencies and run tests

<img src="process_diagrams\cicd.png" alt="drawing" style="width:800px;"/>

- Setup workflow for the QA environment for live testing
- Finally setup workflow for the main Production environment with setting up .env files, and the .zip for cloud upload.

<br/>

**For Production Environment**

- Go to AWS for setting up QA env
- Identify Access Management Dashboard
- Users -> Add User
- Set Access Key for Id and Secret Key
- Go to Attach existing policies directly
- Set AdministrationAccess-AWSElasticBeanstalk
- Create User


**Set Elastic Beanstalk Access For GitHub Actions**

- Set Repository Actions Secrets -> AWS_ACCESS_KEY_ID && AWS_SECRET_ACCESS_KEY
- Create Application in ElasticBeanstalk
- Choose Sample Application and Create Application

- Create a new Node.js action workflow in GitHub Actions
- Set env vars and continuous deployment with AWS

- Workflow will upload zip on its own, when a new commit or merge is done.

- Now, setup a new web server environment on Elastic Beanstalk

- **This setup ensures, on each Pull Request we first run all the unit tests and e2e tests**

- **Deploy it on QA environment, for manual testing**

- **Then deploy on command using _workflow-dispatch_ to Production environment.**

<br/><br/>

**RPC Implementation**
-
- Segregate Responsibilities to follow pure CQRS
    - [Learn about Command and Query Responsibility Segregation](https://learn.microsoft.com/en-us/azure/architecture/patterns/cqrs)
- Optimizing services for scalability
- Refactoring codebase and keeping meaningful data close to the services
- Revoke redundant data from services / DB
<!-- - Isolate DB instance with special access to perform DBA operations -->

<img src="process_diagrams\rpc_communication.png" alt="drawing" style="width:600px;"/>

<br/>

### Refactoring Services to reduce unnecessary connections, references and make them more independent using RPC

- Managing the connections with RPC connections.
- Shifting data endpoints
    - Wishlist and Shopping_Details, from customer service to shopping service.
    - Publishing and sending signals from each service to other to automatically detect, deletion of user to update shopping service and other utilities.
    - Grab product info through RPC from product service to shopping service.
- **This way the shopping service acts as the RPC Request sender and only sends requests for payload to the products service. And the Product service is a RPC Observer that sends payload whenever a request message is received.**
- We can simply reduce the customer model as we no longer need the product and cart and simply can read data from other services using RPC.

<br/><br/>

**Error Handling**
-
- Catch base errors and respond to client with meaningful error code
- Trace / Log Error with suitable tools. example: cloud-watch, Sentry, etc.
- Monitoring & reporting with suitable tools to handle incidents

<br/><br/>

**Testing**
-
- working...

<br/><br/>

**Scaling service using Node.js Clusters**
-
- Implementing Horizontal Scaling and initiating multiple instances of the services using AWS EC2 using Node.js clusters and pm2 :
   [Node.js Cluster Readme](./project-documentation/nodejs.cluster.md)

<br/><br/>


**Scaling Microservices using Reverse Proxy**
-

- Firstly, working with the customer service, we create a docker image of the service and expose it to port 8001.
- A Nginx image uses this port to handle the request from customer service and can be used directly as a container.

<img src="process_diagrams\reverseproxy.png" alt="drawing" style="width:600px;"/>

- Since, nginx is managing all requests for customer service acts as its host. We can simply spin up multiple instance of customer service and the nginx will manage the requests along with load balancing as a reverse proxy.
- We can also change the workflow to use docker instead of pm2 for scaling up customer service.

### Now setting up EC2 Instance for this architecture

- Launch an EC2 instance
  - AMI -> Ubuntu
  - Instance Type -> t2.micro
  - Create key pair
  - Create Security Group
    - All three protocols & My IP Address
  - Storage Type -> Default

- Connect to SSH Client using ```ssh -i "key" "Public DNS"```

- Install Docker, Steps :

```bash
sudo apt install apt-transport-https curl gnupg-agent ca-certificates software-properties-common -y

curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo apt-key add -

sudo add-apt-repository "deb [arch=amd64] https://download.docker.com/linux/ubuntu focal stable"

sudo apt install docker-ce docker-ce-cli containerd.io -y

sudo apt install docker-compose
```

- Next, enable it

```bash
sudo usermod -aG docker $USER

newgrp docker

sudo systemctl enable docker
```

- Now, we can **setup nginx on ec2 and proxy all services by routes**, so that clients can access the services directly by route name, instead of port name.

```bash
sudo apt install nginx

sudo systemctl enable nginx
```

- Now, setup the inbound and outbound rules in the security group.
- Nginx is now setup and we can access the instance via the Public IP address of the instance (currently only http is supported, and not https)

### Setting up the runner for accessing ec2 instance using workflows

- Run the runner commands from the actions page for Linux on GitHub and authenticate the GitHub Actions to the EC2 Instance.
- Now, the runner will listen to changes in the repository. It works along with the actions that have "self-hosted" run time defined.

- To run the runner in the ec2 instance and watch for changes all the time, we install the ```svc.sh``` script and run it.
  
```bash
sudo ./svc.sh install

sudo ./svc.sh start
```

- Setup all the environment variables in the Action Secrets, for workflows to access those variables. 

- Now, setting up the Deploy_EC2 workflow.

### Setting up routes in nginx in the EC2 Instance

- We need to change the nginx config files to setup routes, to edit config files use : ```sudo vim /etc/nginx/sites-available/default```

- Set route locations in the server block in the nginx config and setup headers

```bash
location /customer {
    rewrite ^/customer/(.*)$ /$1 break;
    proxy_pass http://localhost:8001;
    proxy_http_version 1.1;
    proxy_set_header Upgrade $http_upgrade;
    proxy_set_header Connection 'upgrade';
    proxy_set_header Host $host;
    proxy_cache_bypass $http_upgrade;
}
```

- Check the config file for errors ```sudo nginx -t``` and restart nginx ```sudo systemctl restart nginx```

- The service is now working on the given IP address on EC2 with all the defined routes available!
