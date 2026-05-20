Current running instructions
Full dockerisation now in place

First navigate on a lab machine to /src
run ./build.sh to create and save docker image

To run the webpage
run ./run.sh to load and run the docker image

If locally, ./run.sh on a lab machine, access via
http://127.0.0.1:24508

If remotely, ssh to teaching service, navigate to src and ./run.sh
https://mv54.teaching.cs.st-andrews.ac.uk/express/

This requires nginx to be set up with the following
cd /cs/home/$mv54/nginx_default
in nginx.conf
"
location /express/ {
 proxy_pass http://127.0.0.1:24508;
}
"
on remote laptop first do
ssh -ND 1080 mv54@jump.cs.st-andrews.ac.uk