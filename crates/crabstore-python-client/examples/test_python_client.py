from time import sleep
import argparse
import logging
import random
import crabstore_client
import numpy as np


FORMAT = '%(levelname)s %(name)s %(asctime)-15s %(filename)s:%(lineno)d %(message)s'
logging.basicConfig(format=FORMAT)
logging.getLogger().setLevel(logging.INFO)


def alloc(c, size: int):
    oid = crabstore_client.ObjectID.from_binary(b'00000000000000000000')
    b = c.create(oid, size)
    arr = np.frombuffer(b)
    arr[0] = 234
    sleep(0.2)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('sock_path', default='sock_path', type=str)

    args = parser.parse_args()
    c = crabstore_client.CrabClient(args.sock_path)
    oid = crabstore_client.ObjectID.from_binary(b'00000000000000000000')

    """
    Plasma interface:-
    
        buff = client.create(object_id, data_size, metadata)
        array = np.frombuffer(buff, dtype="uint8")
        array[0], array[-1] = 0, -1
    """
    c.connect()

    for i in range(100):
        alloc(c, 2**30)

main()
