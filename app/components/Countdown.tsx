import { Route } from "@/types/Route";
import { Stop } from "@/types/Stop";
import { Vehicle } from "@/types/Vehicle";
import React from "react";
import { Bullet } from "./Bullet";

export function Countdown(props: { routes: Record<string, Route>; stop: Stop; walk_time: number }) {
  if (!props.stop)
    return (
      <React.Fragment>
        <div className="flex min-h-14 flex-row items-center rounded-lg bg-emerald-700"></div>
        <div className="flex min-h-64 flex-row items-center rounded-lg bg-slate-100"></div>
      </React.Fragment>
    );

  const times: Array<Vehicle> = props.stop.trips.filter((vehicle) => {
    return vehicle.minutes_until_arrival > props.walk_time / 2;
  });

  return (
    <React.Fragment>
      <div className="flex min-h-14 flex-row items-center rounded-lg bg-emerald-700">
        <h1 className="mx-2 font-black text-white 2xl:text-3xl">{props.stop.name}</h1>
      </div>
      {times.length > 0 ? (
        <div className="flex min-h-64 flex-row rounded-lg bg-slate-300">
          <div className="flex min-h-full basis-3/4 flex-row rounded-lg bg-slate-200 shadow-2xl">
            <div className="flex min-h-full basis-2/3 rounded-lg bg-slate-100 shadow-2xl">
              <div className="flex min-h-full flex-col px-6 py-2">
                <div className="flex basis-2/5 flex-row items-center">
                  <h1 className="line-clamp-1 font-bold text-black 2xl:text-6xl 2xl:leading-normal">
                    {times[0].destination_name}
                  </h1>
                </div>
                <div className="flex basis-3/5 flex-row items-center gap-4">
                  <Bullet route={props.routes[times[0].route_id]} size={96} />
                  <div className="flex items-baseline">
                    <h1 className="font-bold text-black 2xl:text-9xl">{times[0].minutes_until_arrival}</h1>
                    <h1 className="font-semibold text-black 2xl:text-4xl">min</h1>
                  </div>
                </div>
              </div>
            </div>
            {times[1] ? (
              <div className="flex min-h-full basis-1/3 flex-col px-6 py-2">
                <div className="flex basis-2/5 flex-row items-center">
                  <Bullet route={props.routes[times[1].route_id]} size={72} />
                </div>
                <div className="flex basis-3/5 flex-row items-center gap-4">
                  <div className="flex items-baseline">
                    <h1 className="font-bold text-black 2xl:text-9xl">{times[1].minutes_until_arrival}</h1>
                    <h1 className="font-semibold text-black 2xl:text-4xl">min</h1>
                  </div>
                </div>
              </div>
            ) : undefined}
          </div>
          {times[2] ? (
            <div className="flex min-h-full basis-1/4 flex-col px-6 py-2">
              <div className="flex basis-2/5 flex-row items-center">
                <Bullet route={props.routes[times[2].route_id]} size={72} />
              </div>
              <div className="flex basis-3/5 flex-row items-center gap-4">
                <div className="flex items-baseline">
                  <h1 className="font-bold text-black 2xl:text-9xl">{times[2].minutes_until_arrival}</h1>
                  <h1 className="font-semibold text-black 2xl:text-4xl">min</h1>
                </div>
              </div>
            </div>
          ) : undefined}
        </div>
      ) : (
        <div className="flex min-h-64 flex-row items-center rounded-lg bg-slate-100">
          <h1 className="flex-1 text-center font-bold text-black 2xl:text-5xl">No scheduled vehicles</h1>
        </div>
      )}
    </React.Fragment>
  );
}
