#!/bin/bash
source /opt/opendcs/tsdb_config.sh

cat /dcs_user_dir/user.properties

script -c "manageDatabase -I OpenDCS-Postgres -P /dcs_user_dir/user.properties" <<EOF
${DATABASE_OWNER_USERNAME}
${DATABASE_OWNER_PASSWORD}
1
1
${DATABASE_USERNAME}
${DATABASE_PASSWORD}
${DATABASE_PASSWORD}
EOF
