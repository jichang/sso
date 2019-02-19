import { Component, Input } from "@angular/core";
import { Application } from "../application-model.service";

@Component({
  selector: "applications-list",
  templateUrl: "./applications-list.component.html",
  styleUrls: ["./applications-list.component.css"]
})
export class ApplicationsListComponent {
  @Input() applications: Application[];
}
