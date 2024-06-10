import React from "react";
import { Bullet } from "./Bullet";

export function Calendar() {
  //let name = props.name;

  return (
    <React.Fragment>
      <div className="flex h-full flex-row gap-2">
        <div className="flex h-full basis-1/3 flex-col items-center gap-2">
          <div className="flex w-full grow flex-col items-center gap-2 rounded-lg bg-slate-100 p-2">
            <div className="flex flex-row gap-4">
              <div className="flex h-14.5 items-center justify-center">
                <h1 className="font-bold text-emerald-700 2xl:text-5xl">May</h1>
              </div>
              <Bullet
                route={{ route_id: "", route_name: "7", route_color: "#047857", route_text_color: "#FFFFFF" }}
                size={56}
              ></Bullet>
            </div>
            <div className="line-clamp-2 flex h-21 w-full items-center justify-center rounded-lg bg-emerald-700 px-2">
              <h1 className="text-center font-bold text-slate-100 2xl:text-4xl">AM AP Human Geography</h1>
            </div>
            <div className="line-clamp-2 flex h-21 w-full items-center justify-center rounded-lg bg-emerald-700 px-2">
              <h1 className="text-center font-bold text-slate-100 2xl:text-4xl">AM AP Micro</h1>
            </div>
            <div className="line-clamp-2 flex h-21 w-full items-center justify-center rounded-lg bg-emerald-700 px-2">
              <h1 className="text-center font-bold text-slate-100 2xl:text-4xl">PM AP Statistics</h1>
            </div>
          </div>
        </div>
        <div className="flex h-full basis-1/3 flex-col items-center gap-2">
          <div className="flex w-full grow flex-col items-center gap-2 rounded-lg bg-slate-100 p-2">
            <div className="flex flex-row gap-4">
              <div className="flex h-14.5 items-center justify-center">
                <h1 className="font-bold text-emerald-700 2xl:text-5xl">May</h1>
              </div>
              <Bullet
                route={{ route_id: "", route_name: "8", route_color: "#047857", route_text_color: "#FFFFFF" }}
                size={56}
              ></Bullet>
            </div>
            <div className="line-clamp-2 flex h-21 w-full items-center justify-center rounded-lg bg-emerald-700 px-2">
              <h1 className="text-center font-bold text-slate-100 2xl:text-4xl">AM AP English Lit</h1>
            </div>
            <div className="line-clamp-2 flex h-21 w-full items-center justify-center rounded-lg bg-emerald-700 px-2">
              <h1 className="text-center font-bold text-slate-100 2xl:text-4xl">PM AP Comp Gov</h1>
            </div>
            <div className="line-clamp-2 flex h-21 w-full items-center justify-center rounded-lg bg-emerald-700 px-2">
              <h1 className="text-center font-bold text-slate-100 2xl:text-4xl">PM AP Computer Science</h1>
            </div>
          </div>
        </div>
        <div className="flex h-full basis-1/3 flex-col items-center">
          <div className="flex w-full grow flex-col items-center rounded-lg bg-slate-100">
            <div className="flex w-full grow flex-col items-center gap-2 rounded-lg bg-slate-100 p-2">
              <div className="flex flex-row gap-4">
                <div className="flex h-14.5 items-center justify-center">
                  <h1 className="font-bold text-emerald-700 2xl:text-5xl">May</h1>
                </div>
                <Bullet
                  route={{ route_id: "", route_name: "9", route_color: "#047857", route_text_color: "#FFFFFF" }}
                  size={56}
                ></Bullet>
              </div>
              <div className="line-clamp-2 flex h-21 w-full items-center justify-center rounded-lg bg-emerald-700 px-2">
                <h1 className="text-center font-bold text-slate-100 2xl:text-4xl">AM AP Chinese</h1>
              </div>
              <div className="line-clamp-2 flex h-21 w-full items-center justify-center rounded-lg bg-emerald-700 px-2">
                <h1 className="text-center font-bold text-slate-100 2xl:text-4xl">AM AP Environmental</h1>
              </div>
              <div className="line-clamp-2 flex h-21 w-full items-center justify-center rounded-lg bg-emerald-700 px-2">
                <h1 className="text-center font-bold text-slate-100 2xl:text-4xl">PM AP Pyschology</h1>
              </div>
            </div>
          </div>
        </div>
      </div>
    </React.Fragment>
  );
}
