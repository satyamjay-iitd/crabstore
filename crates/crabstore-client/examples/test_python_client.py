import asyncio
import crabstore_client


def main():
    crabstore_client.sleep("/home/satyam/dev/crabstore/sock")
    # c = crabstore_client.CrabClient("sock")
    # oid = crabstore_client.ObjectID.from_binary(b'00000000000000000000')

    # await c.connect()
    # await c.create(oid, 20, 20)


main()
