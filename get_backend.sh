#!/usr/bin/env bash

quit(){
  echo $*
  exit -1
}

set_cmake(){
  if (($# < 1))
  then
    quit "can't find the corresponding configuration!"
  fi
  echo "set" $1 "as the backend."
  cp -f backends/config/$1/CMakeLists.txt backends/CMakeLists.txt
}

download_if_not_exist(){
  if (($# < 3))
  then
    quit "no enough parameters in download_if_not_exist!"
  fi

  if [ ! -f $2 ];then
    echo "downloading" $1"..."
    curl -R -o $2 $3
  fi
}

unzip_if_not_exist(){
  if (($# < 2))
  then
    quit "no enough parameters in unzip_if_not_exist!"
  fi
  if [ ! -d $1 ];then
    tar $2
  fi
}

if (($# < 1))
then
  quit "please indicate the backend!"
fi

if [ ! -d "backends/bin" ];then
  mkdir backends/bin
fi

backend_name=$1
if [ $backend_name = "lua5.4" ]
then
  download_if_not_exist $backend_name backends/bin/lua5.4.tar.gz http://www.lua.org/ftp/lua-5.4.4.tar.gz
  unzip_if_not_exist "backends/bin/lua5.4.4" "zxf backends/bin/lua5.4.tar.gz -C backends/bin"
else
  quit "backend" $backend_name "is not available!"
fi

set_cmake $backend_name
