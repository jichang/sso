import { Component, OnInit, Input } from "@angular/core";
import { QrCodeConfig } from "../totp.service";

@Component({
  selector: "qrcode",
  templateUrl: "./qrcode.component.html",
  styleUrls: ["./qrcode.component.css"]
})
export class QrcodeComponent implements OnInit {
  @Input() data: QrCodeConfig;

  constructor() {}

  ngOnInit() {}
}
