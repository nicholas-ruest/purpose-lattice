# Deployment and rollback

The committed manifest is Cloud Run-compatible but is not proof of deployment. Bind the runtime service account to only the authorized persistence interface and require the external IAP identity asserted by `PL_AUTH_MODE=external-iap`. The local RVF file proves the prototype path; production must mount/use the authorized RVF persistence service rather than Cloud Run ephemeral disk.

Deploy a source-SHA-tagged image, verify `/readyz`, then route traffic. Roll back with `gcloud run services update-traffic purpose-lattice-api --to-revisions PREVIOUS=100 --region REGION`. No production deployment was authorized for this build.
