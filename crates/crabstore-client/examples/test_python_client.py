from time import sleep
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
    c = crabstore_client.CrabClient("sock")
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

        
    # l = []
    # for i in range(100):
    #     oid = crabstore_client.ObjectID.from_binary(b'00000000000000000000')
    #     b = c.create(oid, 2**30)
    #     arr = np.frombuffer(b)
    #     arr[0] = 234
    #     sleep(0.2)
    #     l.append(b)
    # b = c.create(oid, 2**30)

    # sleep(100)

    # arr = np.frombuffer(b)
    # arr[0] = 234
    # print(arr)


main()
