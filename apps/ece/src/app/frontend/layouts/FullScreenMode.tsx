import React from "react";

export const Box = (): JSX.Element => {
  return (
    <div className="w-[1464px] h-[935px]">
      <div className="fixed w-[1464px] h-[935px] top-0 left-0">
        <div className="relative h-[935px]">
          <div className="absolute w-[1456px] h-[931px] top-[4px] left-[8px] bg-[#fdfbfbb2] rounded-[16px] shadow-[0px_2.77px_2.21px_#62626205]" />
          <div className="absolute w-[1456px] h-[917px] top-[18px] left-[8px]">
            <div className="absolute w-[730px] h-[259px] top-[255px] left-[377px]">
              <div className="w-[734px] h-[259px]">
                <div className="relative w-[730px] h-[259px]">
                  <div className="absolute w-[220px] h-[39px] top-[220px] left-[256px] [font-family:'Inter-Bold',Helvetica] font-bold text-black text-[16px] text-center tracking-[0] leading-[normal]">
                    Type “START” Below:
                  </div>
                  <div className="absolute w-[730px] h-[240px] top-0 left-0 [font-family:'Inter-Bold',Helvetica] font-bold text-[#3b3b3bb2] text-[96px] text-center tracking-[0] leading-[normal]">
                    Nano Node
                  </div>
                </div>
              </div>
            </div>
            <div className="absolute w-[1456px] h-[52px] top-[865px] left-0 bg-[#fdfbfb4c] border-[3px] border-solid border-[#0000004c]">
              <div className="relative w-[89px] h-[17px] top-[15px] left-[18px]">
                <div className="absolute w-[69px] h-[17px] top-0 left-[16px] [font-family:'Inter-Bold',Helvetica] font-bold text-[#3b3b3bb2] text-[16px] tracking-[0] leading-[normal] whitespace-nowrap">
                  START
                </div>
                <div className="absolute w-[7px] h-[17px] top-0 left-0 [font-family:'Inter-Bold',Helvetica] font-bold text-black text-[16px] tracking-[0] leading-[normal] whitespace-nowrap">
                  :
                </div>
              </div>
            </div>
            <div className="absolute w-[292px] h-[15px] top-0 left-[601px]">
              <div className="relative w-[298px] h-[15px]">
                <div className="absolute w-[298px] h-[15px] top-0 left-0">
                  <div className="absolute h-[15px] top-0 left-[227px] [font-family:'Inter-Bold',Helvetica] font-bold text-black text-[12px] tracking-[0] leading-[normal]">
                    Nano Node
                  </div>
                  <div className="absolute h-[15px] top-0 left-[111px] [font-family:'Inter-Bold',Helvetica] font-bold text-black text-[12px] tracking-[0] leading-[normal]">
                    TXN Tracker
                  </div>
                  <div className="absolute h-[15px] top-0 left-0 [font-family:'Inter-Bold',Helvetica] font-bold text-black text-[12px] tracking-[0] leading-[normal]">
                    SocketChat
                  </div>
                </div>
                <img
                  className="absolute w-[10px] h-[10px] top-[3px] left-[216px]"
                  alt="Menu toggle"
                  src="menu-toggle.png"
                />
              </div>
            </div>
          </div>
          <div className="absolute w-[1454px] h-[924px] top-0 left-0">
            <img
              className="absolute w-[30px] h-[30px] top-[894px] left-[1424px]"
              alt="Full screen toggle"
              src="full-screen-toggle.png"
            />
            <img
              className="absolute w-[30px] h-[30px] top-[10px] left-[1424px]"
              alt="Dark mode toggle"
              src="dark-mode-toggle.png"
            />
            <img className="absolute w-[109px] h-[56px] top-0 left-0" alt="Logo light mode" src="logo-light-mode.png" />
          </div>
        </div>
      </div>
    </div>
  );
};
