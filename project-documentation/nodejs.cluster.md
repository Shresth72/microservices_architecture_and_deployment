# Scaling service using Node.js Clusters
-
- Implementing Horizontal Scaling and initiating multiple instances of the services using AWS EC2 using Node.js clusters and pm2 :
   [Node.js Cluster Readme](./project-documentation)
   <!-- - <br/> -->
   <!-- <img src="project-documentation/images/image.png" alt="drawing" style="width:400px;"/> -->
    - Launch Instance
    - Choose the Amazon Machine Image, and select the OS image
    - Choose Instance type
    - Create Key Pair to securely connect to the instance
    - Choose Security Group (SSH traffic, allows all IP addresses to access the instance)
- Create the Instance and Connect
- Connecting using SSH client :
    - Open an SSH client
    - Locate your private key file. The key for this instance for e.g. ```k8s-access.com```
    - Run this command to ensure the key is not publicly available.
    ```bash 
    chmod 400 k8s-access.pem
    ``` 
    - Connect to the instance using its Public DNS. For e.g. ```ec2-52-58-237-8.eu-central-1.compute.amazonaws.com```
  
- Run this SSH command to connect
```bash
ssh -i "key" "public DNS"
```
<br/>

### Setting up connection to GitHub repo of the service

- Install node and git on the instance

- Now generate SSH key pair to connect it the GitHub repo using :
```bash
ssh-keygen -t rsa -C "email" -b 4096
```

- The private and public key are saved in your instance. Grab the public key using : 
```bash
cat ~/.ssh/id_rsa.pub
```

- Add the public key in Deploy Keys on GitHub. To check if the key can authenticate the GitHub to the ec2 instance, run ```ssh -vT git@github.com```

- If it perfectly connects, clone the git repo onto the ec2 instance.Install all the dependencies and export all .env variables too in the cloned repo and run it. Also, run the scale command to expose multiple pId's of your instance.

- To connect and control traffic directly from the ec2 console and expose a Port to connect; add inbound security rules in the security groups. Create a custom TCP rule with "PORT" value and "Anywhere IPv4" as source to access the URL from anywhere.
  
<br/>

### Making connection secure using NGINX
- Delete previous custom TCP rule and setup a new HTTP and HTTPS rules
- Install Nginx on the ec2 instance and enable it.
```bash
sudo amazon-linux-extras install nginx1 -y

sudo systemctl enable nginx

sudo systemctl start nginx

sudo systemctl status nginx
```

- To reach the application server, edit the nginx config file
```bash
sudo vim /etc/nginx/nginx.config
```

- Setup a proxy pass to the port in the config file in the server block
```yml
location / {
    proxy_pass http://localhost:8001;
}
```

- Check the config file for errors ```sudo nginx -t``` and restart nginx ```sudo systemctl restart nginx```

<br/><br/>

**Setting up workflows to automate this whole process**
-

- Setup a self-hosted workflow for deploying to ec2 on a node.js run environment
- Setup a new self-hosted runner for GitHub Actions
  - Choose Linux
  - And run all the commands in the runner on the ec2 console.
- This creates a service file ```svc.sh```, which needs to be installed and started. 

**This service file is responsible for adding a listener that listens for the actions in the GitHub repo with the help of the Linux runner and it will directly deploy it to the EC2 Instance on AWS**
```bash
sudo ./svc.sh install

sudo ./svc.sh start
```

<br/>
<hr/>
<hr/>
<br/>
