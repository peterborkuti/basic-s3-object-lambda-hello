# AWS S3 Object Lambda Test

It uses a GetObject event and it returns with the text "Hello Rust" instead of the real
object from the S3 bucket.

## Build & Deploy

1. Install [cargo-lambda](https://github.com/cargo-lambda/cargo-lambda#installation)
2. Build the function with `cargo lambda build --release --arm64 --output-format zip`
3. Upload the bootstrap.zip file from the directory:`target/lambda/basic-s3-object-lambda-hello`

## Setup on AWS S3

1. You need a bucket and upload a file to that bucket
2. Set Access Point for that bucket
3. Set Object Lambda Access Point for the access point and use the uploaded lambda function as a transformer

## Set Up on AWS Lambda

0. Click on Code tab
1. Runtime settings - runtime: Custom runtime on Amazon Linux 2
2. Runtime settings - Architecture: arm64

## Set Up on AWS IAM

1. Click on Roles
2. Search the lambda function name
3. Add the permission: AmazonS3ObjectLambdaExecutionRolePolicy

## How to check this lambda

1. Go to S3
2. Click on Object Lambda Access Point
3. Click on your object lambda access point name
4. click on one uploaded file
5. Click on the activated Open button

### Expected:
A new browser tab opens with the text "Hello rust"

### Experienced:
A new browser tab opens with the text:

```
<Error>
<link type="text/css" id="dark-mode" rel="stylesheet" href=""/>
<Code>LambdaResponseNotReceived</Code>
<Message>The Lambda exited without successfully calling WriteGetObjectResponse.</Message>
<RequestId>1ef7ce68-77ec-4155-9950-86820ee8a92a</RequestId>
<HostId>{host-id}</HostId>
</Error>
```
#### The trace info with the error:

```
DEBUG resolving host="s3-object-lambda.us-east-2.amazonaws.com"

TRACE Lambda runtime invoke{requestId="04c50f91-116e-4155-b744-b3e7553dc6fe" xrayTraceId="Root=1-6431b558-19be6d894b8d7b025bf628ee;Parent=4e1e7e0a53860a04;Sampled=0;Lineage=48645c91:0"}:send_operation{operation="WriteGetObjectResponse" service="s3"}:dispatch: checkout dropped for ("https", s3-object-lambda.us-east-2.amazonaws.com)

TRACE Lambda runtime invoke{requestId="04c50f91-116e-4155-b744-b3e7553dc6fe" xrayTraceId="Root=1-6431b558-19be6d894b8d7b025bf628ee;Parent=4e1e7e0a53860a04;Sampled=0;Lineage=48645c91:0"}:send_operation{operation="WriteGetObjectResponse" service="s3"}: No smithy connection found! The underlying HTTP connection never set a connection.

TRACE Lambda runtime invoke{requestId="04c50f91-116e-4155-b744-b3e7553dc6fe" xrayTraceId="Root=1-6431b558-19be6d894b8d7b025bf628ee;Parent=4e1e7e0a53860a04;Sampled=0;Lineage=48645c91:0"}:send_operation{operation="WriteGetObjectResponse" service="s3"}: retry classification retry_kind=Error(TransientError)
```


## You can test this lambda with this event

```
{
    "xAmzRequestId": "1a5ed718-5f53-471d-b6fe-5cf62d88d02a",
    "getObjectContext": {
        "inputS3Url": "https://myap-123412341234.s3-accesspoint.us-east-1.amazonaws.com/s3.txt?X-Amz-Security-Token=...",
        "outputRoute": "io-iad-cell001",
        "outputToken": "OUTPUTTOKEN"
    },
    "configuration": {
        "accessPointArn": "arn:aws:s3-object-lambda:us-east-1:123412341234:accesspoint/myolap",
        "supportingAccessPointArn": "arn:aws:s3:us-east-1:123412341234:accesspoint/myap",
        "payload": "test"
    },
    "userRequest": {
        "url": "/s3.txt",
        "headers": {
            "Host": "myolap-123412341234.s3-object-lambda.us-east-1.amazonaws.com",
            "Accept-Encoding": "identity",
            "X-Amz-Content-SHA256": "e3b0c44297fc1c149afbf4c8995fb92427ae41e4649b934ca495991b7852b855"
        }
    },
    "userIdentity": {
        "type": "IAMUser",
        "principalId": "PRNCIPALID",
        "arn": "arn:aws:iam::123412341234:user/myuser",
        "accountId": "123412341234",
        "accessKeyId": "ACCESSKEY"
    },
    "protocolVersion": "1.00"
}

```

## A similar python function which works

```
import boto3

def lambda_handler(event, context):
    print(event)

    object_get_context = event["getObjectContext"]
    request_route = object_get_context["outputRoute"]
    request_token = object_get_context["outputToken"]

    s3 = boto3.client('s3')
    response = s3.write_get_object_response(
        Body="Hello",
        RequestRoute=request_route,
        RequestToken=request_token)
    
    print("Response: " + str(response))

    return {'status_code': 200}
```
