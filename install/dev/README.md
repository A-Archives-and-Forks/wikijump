## DEV docker-compose

For a permanent dev deployment, you can run docker-compose to start the provided containers. Feel free to modify this configuration to suit your purposes.

The main infrastructural requirement is for two S3 buckets. Information about how to access external resources must be passed as environment variables.
Presently, Postgres and Valkey (Redis) are exposed as Docker images. If you wish, you can replace these with external services by updating the appropriate environment variables.

Referring to S3 is a bit unusual, because which variables are required differ depending on the provider. For any container which requires S3 access, all of the required S3 variables must be provided so the system knows how to connect to the S3 service.

| Environment Variable    | Required              | Description |
|-------------------------|-----------------------|-------------|
| `S3_FILES_BUCKET`       | Depends on container. | The name of the bucket where uploaded files and avatars are kept. |
| `S3_TEXT_BLOCKS_BUCKET` | Depends on container. | The name of the bucket where hosted text blocks are kept. |
| `S3_AWS_REGION`         | If using AWS S3.      | The AWS region this bucket is in. |
| `S3_REGION_NAME`        | If not using AWS S3.  | The region this bucket is in. |
| `S3_CUSTOM_ENDPOINT`    | If not using AWS S3.  | The S3 endpoint to connect to. |
| `S3_PATH_STYLE`         | Always.               | Boolean. Reflects whether this S3 service expects requests to be [path-style](https://docs.aws.amazon.com/AmazonS3/latest/userguide/VirtualHosting.html#path-style-access) (true) or [virtual-host-style](https://docs.aws.amazon.com/AmazonS3/latest/userguide/VirtualHosting.html#virtual-hosted-style-access) (false). Some services accept both. |
| `S3_ACCESS_KEY_ID`      | If not using an AWS profile file. | S3 credentials. |
| `S3_SECRET_ACCESS_KEY`  | If not using an AWS profile file. | S3 credentials. |
| `AWS_PROFILE_NAME`      | If providing credentials via AWS profile file. | The name of the AWS profile to read credentials from. |

* Container `deepwell` requires Postgres, Valkey, and both S3 buckets.
* Container `wws` requires Valkey and both S3 buckets.
