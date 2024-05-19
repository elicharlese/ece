import { useState } from "react";

export const NanoNode  => () {

start = (
<div className="flex flex-col items-center w-[1112px] box-sizing-border">
  <div className="m-[0_0_112px_1px] flex flex-col items-center w-[fit-content] box-sizing-border">
    <div className="m-[0_0_12px_0] inline-block break-words font-['Inter'] font-bold text-[96px] text-[rgba(60,60,60,0.7)]">
      Nano Node
    </div>
    <span className="break-words font-['Inter'] font-bold text-[16px] text-[#000000]">
      Type “START” Below
    </span>
  </div>
  <div className="rounded-b-[16px] border-t-[3px_solid_rgba(98,98,98,0.3)] bg-[rgba(253,251,251,0.3)] relative flex flex-row justify-between p-[15px_11px_14px_16px] w-[1112px] box-sizing-border">
    <div className="m-[5px_0_6px_0] flex flex-row box-sizing-border">
      <span className="m-[0_3.5px_0_0] break-words font-['Inter'] font-bold text-[16px] text-[#000000]">
        : 
      </span>
      <span className="break-words font-['Inter'] font-bold text-[16px] text-[rgba(60,60,60,0.7)]">
        START
      </span>
    </div>
    <div className="bg-[url('assets/images/FullScreenToggle1.png')] bg-[50%_50%] bg-contain bg-no-repeat w-[30px] h-[30px]">
    </div>
  </div>
</div>
);

NanoNode = (
  <div className="shadow-[0px_2.8px_2.2px_0px_rgba(98,98,98,0.02)] rounded-[16px] bg-[rgba(253,251,251,0.7)] relative flex flex-col items-center p-[14px_0_0px_0] box-sizing-border">
    <div className="m-[0_0_23px_37.4px] flex flex-row justify-between w-[291.4px] box-sizing-border">
      <span className="m-[0_7.5px_0_0] w-[103.5px] break-words font-['Inter'] font-bold text-[12px] text-[#000000]">
        SocketChat
      </span>
      <span className="break-words font-['Inter'] font-bold text-[12px] text-[#000000]">
        TXN Tracker
      </span>
      <div className="flex flex-row box-sizing-border">
        <div className="bg-[url('assets/images/MenuToggle.png')] bg-[50%_50%] bg-contain bg-no-repeat m-[3px_1px_2px_0] w-[10px] h-[10px]">
        </div>
        <span className="break-words font-['Inter'] font-bold text-[12px] text-[#000000]">
          Nano Node
        </span>
      </div>
    </div>
    <div className="relative flex p-[26px_0_0_0] box-sizing-border">
      <div className="relative flex flex-col items-center box-sizing-border">
        <div className="flex flex-row w-[fit-content] box-sizing-border">
          <div className="bg-[rgba(0,0,0,0.8)] relative p-[16px_14.8px_0_14.8px] w-[741.4px] h-[769px] box-sizing-border">
            <span className="break-words font-['Inter'] font-bold text-[16px] text-[#FFFFFF]">
              &gt;&gt;&gt;
            </span>
          </div>
          <div className="bg-[rgba(0,0,0,0.9)] relative p-[17px_27.7px_0_27.7px] w-[714.6px] h-[769px] box-sizing-border">
            <div className="relative p-[91px_0_55px_0] box-sizing-border">
              <span className="relative break-words font-['Inter'] font-bold text-[16px] text-[#FFFFFF]">
                07:40:24 - time_synchronizer - Could not refresh server time. Check network <br />
                connection.
              </span>
              <span className="absolute left-[50%] bottom-[0px] translate-x-[-50%] break-words font-['Inter'] font-bold text-[16px] text-[#FFFFFF]">
                07:40:24 - BinanceExchange - There was an error requesting exchange info. ( See <br />
                log file or stack trace dump )
              </span>
              <span className="absolute left-[0px] top-[36px] break-words font-['Inter'] font-bold text-[16px] text-[#FFFFFF]">
                07:40:24 - time_synchronizer - Error getting server time. ( See log file  f or stack <br />
                trace dump )
              </span>
              <span className="absolute left-[0px] top-[0px] break-words font-['Inter'] font-bold text-[16px] text-[#FFFFFF]">
                Running logs
              </span>
            </div>
          </div>
        </div>
        <div className="bg-[#000000] relative flex flex-row justify-between p-[3.8px_1.6px_7px_4px] w-[1456px] box-sizing-border">
          <div className="m-[2.2px_9.5px_0_0] inline-block w-[777.5px] break-words font-['Inter'] font-bold text-[16px] text-[rgba(255,255,255,0.7)]">
            Trades: 0, Total P&amp;L: 0.00, Return %: 0.00%
          </div>
          <div className="m-[0_0_2.2px_0] inline-block break-words font-['Inter'] font-bold text-[16px] text-[rgba(255,255,255,0.7)]">
            CPU:  7.0%, Mem:  71.91 MB  (241.68 MB),  Threads:   13,  Uptime:   0 day(s),   00:00:09
          </div>
        </div>
      </div>
      <div className="bg-[#000000] absolute left-[0px] top-[0px] right-[0px] flex flex-row justify-between p-[7px_25.5px_6px_15px] box-sizing-border">
        <span className="m-[0_9.5px_0_0] w-[290.5px] break-words font-['Inter'] font-bold text-[16px] text-[rgba(255,255,255,0.7)]">
          Version: 1.21.0
        </span>
        <span className="break-words font-['Inter'] font-bold text-[16px] text-[#FFFFFF]">
          Strategy: None
        </span>
        <span className="break-words font-['Inter'] font-bold text-[16px] text-[#FFFFFF]">
          Strategy File: None
        </span>
        <span className="break-words font-['Inter'] font-bold text-[16px] text-[#FFFFFF]">
          Gateway: OFFLINE
        </span>
        <span className="break-words font-['Inter'] font-bold text-[16px] text-[#FFFFFF]">
          &gt; log pane
        </span>
      </div>
    </div>
    <div className="rounded-b-[16px] border-[3px_solid_rgba(0,0,0,0.3)] bg-[rgba(253,251,251,0.3)] relative flex flex-row p-[15.4px_0_17.6px_20.9px] w-[1456px] box-sizing-border">
      <span className="m-[0_7.2px_0_0] break-words font-['Inter'] font-bold text-[16px] text-[#000000]">
        : 
      </span>
      <span className="break-words font-['Inter'] font-bold text-[16px] text-[rgba(60,60,60,0.7)]">
        START
      </span>
    </div>
  </div>
)