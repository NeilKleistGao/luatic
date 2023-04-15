#!/usr/bin/env bash

quit(){
  echo $*
  exit -1
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

if [ ! -d "backends/bin" ];then
  mkdir backends/bin
fi

# lua 5.4 vm
download_if_not_exist "lua5.4" backends/bin/lua5.4.tar.gz http://www.lua.org/ftp/lua-5.4.4.tar.gz
unzip_if_not_exist "backends/bin/lua5.4.4" "zxf backends/bin/lua5.4.tar.gz -C backends/bin"
