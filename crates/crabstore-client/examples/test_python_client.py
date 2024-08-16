import asyncio
import crabstore_client


async def main():
    c = crabstore_client.CrabClient("sock")
    oid = crabstore_client.ObjectID.from_binary(b'00000000000000000000')

    await c.connect()
    await c.create(oid, 20, 20)


asyncio.run(main())
