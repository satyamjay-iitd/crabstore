import logging
import crabstore_client


FORMAT = '%(levelname)s %(name)s %(asctime)-15s %(filename)s:%(lineno)d %(message)s'
logging.basicConfig(format=FORMAT)
logging.getLogger().setLevel(logging.INFO)


def main():
    c = crabstore_client.CrabClient("sock")
    oid = crabstore_client.ObjectID.from_binary(b'00000000000000000000')

    print(c.connect())
    print(c.create(oid, 20, 20))


main()
