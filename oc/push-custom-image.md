# How to push an image to OpenShift

* Bring up the cluster 

    oc cluster up

* Create a pusher service account

    oc create serviceaccount pusher
    oc policy add-role-to-user edit system:serviceaccount:myproject:pusher

* Create the image stream to push to

    oc create -f imagestream.yaml

* Expose the OpenShift docker registry

    oc login -u system:admin
    oc expose service docker-registry -n default

* Login to the docker registry 
    You can see the registry IP when logging in to the web console, and looking at the image stream

    oc describe sa pusher
    (look for token id)
    oc describe secret ${SOME_TOKEN_ID}
    docker login --username=bjolof 172.30.1.1:5000
    (enter token)

* Tag and push your imgae
    docker images
    docker tag ${SOME_HASH} 172.30.1.1:5000/myproject/rust-musl-pipedream:latest
    docker push 172.30.1.1:5000/myproject/rust-musl-pipedream:latest

# Misc commands

    oc status

  Import latest upstream builder image:

    oc import-image rust-musl:1.0