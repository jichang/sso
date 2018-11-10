import { Component } from "@angular/core";
import { MatIconRegistry } from "@angular/material";
import { DomSanitizer } from "@angular/platform-browser";

@Component({
  selector: "app-root",
  templateUrl: "./app.component.html",
  styleUrls: ["./app.component.css"]
})
export class AppComponent {
  constructor(
    private iconRegistry: MatIconRegistry,
    private domSatinizer: DomSanitizer
  ) {
    let iconUrl = domSatinizer.bypassSecurityTrustResourceUrl(
      "/assets/logo.svg"
    );
    iconRegistry.addSvgIcon("logo", iconUrl);
  }
}
