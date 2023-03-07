
export function name() {
    return 'Rust';
}

export class DataGridModel {
    constructor(id, width, height, rows = 100, columns = 50) {
        this._number = 42;
        this._gridId = id;
        this._grid = null;
        this._height = height;
        this._width = width;
        this._rows = rows;
        this._columns = columns;

        this.init(id, width, height);
    }

    get number() {
        return this._number;
    }

    set number(n) {
        return this._number = n;
    }

    get gridId() {
        return this._gridId;
    }

    set gridId(id) {
        return this._gridId = id;
    }

    render() {
        return `My number is: ${this.number}`;
    }

    async init(id, width, height) {
        this._grid = await this.canvasResolver(id);
        const excelSheet = this.excelSheet();
        this._grid.data = excelSheet.data;

        this._grid.attributes.columnHeaderClickBehavior = 'select';
        this._grid.style.columnHeaderCellHorizontalAlignment = 'center';
        this._grid.style.width = `${width}`;
        this._grid.style.height = `${height}`;
        
    }

    canvasResolver(id) {
        return new Promise(resolve => {
            setTimeout(() => {
                resolve(document.getElementById(id));
            }, 10); // 20 = 20 miliseconds, 2000 = 2seconds
        });
    }

    excelSheet() {
        let data = [];
        
        const getColumnName = (n) => {
          const ordA = 'a'.charCodeAt(0);
          const ordZ = 'z'.charCodeAt(0);
          const len = ordZ - ordA + 1;
          let s = '';
      
          while (n >= 0) {
            s = String.fromCharCode((n % len) + ordA) + s;
            n = Math.floor(n / len) - 1;
          }
      
          return s;
        }
        
        if (this._grid.data !== undefined) {
            for (let x = 0; x < rows; x += 1) {
                let row = {};
            
                for (let y = 0; y < cols; y += 1) {
                  const n = getColumnName(y).toUpperCase();
                  row[n] = x * y;
                }
            
                data.push(row);
            }
        }
        else {
            for (let x = 0; x < this._rows; x += 1) {
                let row = {};
            
                for (let y = 0; y < this._columns; y += 1) {
                  const n = getColumnName(y).toUpperCase();
                  row[n] = "";
                }
            
                data.push(row);
            }
        }
      
        return {data};
    }
      
}
