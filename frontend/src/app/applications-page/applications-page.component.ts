import { Component, OnInit, OnDestroy } from "@angular/core";
import { Router } from "@angular/router";
import {
  Application,
  ApplicationModelService
} from "../application-model.service";
import { session } from "../model";
import { Subscription } from "rxjs";

@Component({
  selector: "applications-page",
  templateUrl: "./applications-page.component.html",
  styleUrls: ["./applications-page.component.css"]
})
export class ApplicationsPageComponent implements OnInit, OnDestroy {
  applications: Application[] = [];
  subscription: Subscription;

  constructor(
    private router: Router,
    private applicationModel: ApplicationModelService
  ) {}

  ngOnInit() {
    this.subscription = this.applicationModel.applications.subscribe(
      applications => {
        this.applications = applications;
      }
    );

    let currUser = session.currUser();
    if (currUser) {
      this.applicationModel.select(currUser.id);
    } else {
      this.router.navigate(["login"]);
    }
  }

  ngOnDestroy() {
    this.subscription.unsubscribe();
  }

  openCreateModal() {}
}
