import { Component, OnInit, Input } from "@angular/core";
import { Router, ActivatedRoute } from "@angular/router";
import {
  Application,
  ApplicationModelService
} from "../application-model.service";
import { map } from "rxjs/operators";

@Component({
  selector: "application-basic",
  templateUrl: "./application-basic.component.html",
  styleUrls: ["./application-basic.component.css"]
})
export class ApplicationBasicComponent implements OnInit {
  @Input() application: Application;

  constructor() {}

  ngOnInit() {}
}
