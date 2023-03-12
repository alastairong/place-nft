import { Placement, DestructuredPlacement } from "./types";

export function destructurePlacement(placement: Placement): DestructuredPlacement {
    let id: number = placement.pixel >> 4;
    return {
      x: id >> 16,
      y: id % (1 << 16),
      colorIndex: placement.pixel % 16
    };
  }
  
  
  export function packPlacement(destructured: DestructuredPlacement): Placement {
    let pixel = destructured.colorIndex
    + destructured.x << 20
        + destructured.y << 4;
    return {
      pixel
    };
  }

  export function updateGrid(imageData: Uint8Array, placements: Placement[]): String[] {
    if(imageData.length != Math.floor(128 * 128 / 2)) {
      console.error(`snapshotIntoFrame(${128}) error: invalid imageData length: ` + imageData.length)
      return new Array();
    }
    let grid = new Array(128 * 128);
    // grid.fill("#FFFFFF");
    let i = 0;

    // Render the base snapshot
    for (const packed of imageData) {
      const doublePixel = intoDoublePixel(packed);
      let color1 = COLOR_PALETTE[doublePixel[1]];
      let color2 = COLOR_PALETTE[doublePixel[0]];
  
      grid[i] = color1; // R
      grid[i + 1] = color2;  // G

      i += 1;
    }
    // Apply placements made since that snapshot on top
    for (const placement of placements) {
      const { x, y, colorIndex } = destructurePlacement(placement);
      grid[x + y * 128] = COLOR_PALETTE[colorIndex];
    }

    return grid;
  }

  export type DoublePixel = [upper: number, lower: number];

  function intoDoublePixel(packed: number): DoublePixel {
    return [Math.floor(packed / 16), packed % 16];
  }

  export const COLOR_PALETTE = [
    "#FFFFFF",
    "#E4E4E4",
    "#888888",
    "#222222",
    "#FDA1D3",
    "#F82200",
    "#F09200",
    "#A86839",
    "#E6DA00",
    "#7BE400",
    "#0FC300",
    "#34D7E0",
    "#2B84CD",
    "#3200F4",
    "#DE64EA",
    "#8E0A85",
  ];
  
  
  export function color2index(color:string): number {
    switch(color) {
      case "#FFFFFF": return 0; break;
      case "#E4E4E4": return 1; break;
      case "#888888": return 2; break;
      case "#222222": return 3; break;
      case "#FDA1D3": return 4; break;
      case "#F82200": return 5; break;
      case "#F09200": return 6; break;
      case "#A86839": return 7; break;
      case "#E6DA00": return 8; break;
      case "#7BE400": return 9; break;
      case "#0FC300": return 10; break;
      case "#34D7E0": return 11; break;
      case "#2B84CD": return 12; break;
      case "#3200F4": return 13; break;
      case "#DE64EA": return 14; break;
      case "#8E0A85": return 15; break;
    }
    return 0;
  }
  
