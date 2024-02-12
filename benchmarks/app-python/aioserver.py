import sys
import asyncio
import aiohttp
from aiohttp import web

routes = web.RouteTableDef()

@routes.get('/')
@routes.get('/hello')
async def hello(request):
    return web.Response(text="Hello from python\n")

async def callback(url):
    print("Invoking callback at: ", url)
    async with aiohttp.ClientSession() as session:
        async with session.get(url) as resp:
            print('Response: ', resp.status)

async def main():
    app = web.Application()
    app.add_routes(routes)
    # web.run_app(app)

    runner = web.AppRunner(app)
    await runner.setup()

    site = web.TCPSite(runner, '0.0.0.0', 8080)
    await site.start()
    print("Server listening on 0.0.0.0:8080")
    url = sys.argv[1]
    await callback(url)

    while True:
        await asyncio.sleep(3600)  # sleep forever

asyncio.run(main())

