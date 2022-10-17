#!/bin/sh
set -e

# Install dependencies needed by our wheel
yum -y -q -e 0 install gcc libffi-devel

# upgrade wheel
$LINUX32 /opt/python/cp36-cp36m/bin/pip install --upgrade wheel

# Build wheels
if [ "$AUDITWHEEL_ARCH" == "i686" ]; then
  LINUX32=linux32
fi

$LINUX32 /opt/python/cp36-cp36m/bin/python setup.py bdist_wheel

# Audit wheels
for wheel in dist/*-linux_*.whl; do
  auditwheel repair $wheel -w dist/
  rm $wheel
done
