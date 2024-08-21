import logging
import random
import crabstore_client
import numpy as np


FORMAT = '%(levelname)s %(name)s %(asctime)-15s %(filename)s:%(lineno)d %(message)s'
logging.basicConfig(format=FORMAT)
logging.getLogger().setLevel(logging.INFO)


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
    b = c.create(oid, 256)

    arr = np.frombuffer(b)
    arr[0] = random.randint(0, 255)


main()
