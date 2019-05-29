import { Component, OnInit } from "@angular/core";
import { Subscription } from "rxjs";
import { UsersStore, UsersModelService } from "../users-model.service";
import { session, PaginatorParams } from "../model";
import { Router } from "@angular/router";
import { PageEvent } from "@angular/material/paginator";
import { MatSnackBar } from "@angular/material/snack-bar";

@Component({
  selector: "users-page",
  templateUrl: "./users-page.component.html",
  styleUrls: ["./users-page.component.css"]
})
export class UsersPageComponent implements OnInit {
  pageSizeOptions: number[] = [5, 10, 25, 100];
  paginatorParams: PaginatorParams = {
    limit: 20,
    offset: 0
  };
  store: UsersStore = {
    users: [],
    total: 0
  };
  usersSubscription: Subscription;

  constructor(
    private userModel: UsersModelService,
    private router: Router,
    private snackBar: MatSnackBar
  ) {}

  ngOnInit() {
    this.usersSubscription = this.userModel.users.subscribe(
      store => {
        this.store = store;
      },
      err => {
        if (err.status === 403) {
          this.snackBar.open("action is forbidden", "Dismiss", {
            duration: 3000
          });
        }
      }
    );

    let currUser = session.currUser();
    if (currUser) {
      this.userModel.select(this.paginatorParams);
    } else {
      this.router.navigate(["signin"]);
    }
  }

  changePage(page: PageEvent) {
    this.paginatorParams.limit = page.pageSize;
    this.paginatorParams.offset = 0;

    this.userModel.select(this.paginatorParams);
  }
}
