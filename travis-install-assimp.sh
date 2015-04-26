#!/bin/sh
set -ex
wget https://github.com/assimp/assimp/archive/v3.1.1.tar.gz
tar -xzvf v3.1.1.tar.gz
cd assimp-3.1.1 && cmake . && make && sudo make install && cd ..
